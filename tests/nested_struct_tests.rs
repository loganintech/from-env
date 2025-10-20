use from_env::{FromEnv, FromEnvTrait};

// Nested struct for testing
#[derive(Debug, FromEnv, PartialEq)]
struct Listen {
    address: String,
    port: u32,
}

// Parent struct with required nested struct
#[derive(Debug, FromEnv, PartialEq)]
struct ServerConfigRequired {
    disabled: bool,
    listener: Listen,
}

#[test]
fn test_nested_struct_required() {
    // Set env vars for parent struct
    std::env::set_var("SERVERCONFIGREQUIRED_DISABLED", "false");

    // Set env vars for nested struct - these should have the full prefix
    std::env::set_var("SERVERCONFIGREQUIRED_LISTENER_ADDRESS", "0.0.0.0");
    std::env::set_var("SERVERCONFIGREQUIRED_LISTENER_PORT", "8080");

    let result = ServerConfigRequired::from_env();
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());

    let config = result.unwrap();
    assert_eq!(config.disabled, false);
    assert_eq!(config.listener.address, "0.0.0.0");
    assert_eq!(config.listener.port, 8080);

    // Clean up
    std::env::remove_var("SERVERCONFIGREQUIRED_DISABLED");
    std::env::remove_var("SERVERCONFIGREQUIRED_LISTENER_ADDRESS");
    std::env::remove_var("SERVERCONFIGREQUIRED_LISTENER_PORT");
}

// Parent struct with optional nested struct
#[derive(Debug, FromEnv, PartialEq)]
struct ServerConfigOptional {
    disabled: bool,
    listener: Option<Listen>,
}

#[test]
fn test_nested_struct_optional_present() {
    std::env::set_var("SERVERCONFIGOPTIONAL_DISABLED", "false");
    std::env::set_var("SERVERCONFIGOPTIONAL_LISTENER_ADDRESS", "127.0.0.1");
    std::env::set_var("SERVERCONFIGOPTIONAL_LISTENER_PORT", "9090");

    let result = ServerConfigOptional::from_env();
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());

    let config = result.unwrap();
    assert_eq!(config.disabled, false);
    assert!(config.listener.is_some());

    let listener = config.listener.unwrap();
    assert_eq!(listener.address, "127.0.0.1");
    assert_eq!(listener.port, 9090);

    // Clean up
    std::env::remove_var("SERVERCONFIGOPTIONAL_DISABLED");
    std::env::remove_var("SERVERCONFIGOPTIONAL_LISTENER_ADDRESS");
}

#[test]
fn test_nested_struct_optional_absent() {
    std::env::set_var("SERVERCONFIGOPTIONAL2_DISABLED", "true");
    // Don't set any LISTENER env vars
    std::env::remove_var("SERVERCONFIGOPTIONAL2_LISTENER_ADDRESS");
    std::env::remove_var("SERVERCONFIGOPTIONAL2_LISTENER_PORT");


    let result = ServerConfigOptional2::from_env();
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());

    let config = result.unwrap();
    assert_eq!(config.disabled, true);
    assert!(config.listener.is_none());

    std::env::remove_var("SERVERCONFIGOPTIONAL2_DISABLED");
}

#[derive(Debug, FromEnv, PartialEq)]
struct ServerConfigOptional2 {
    disabled: bool,
    listener: Option<Listen>,
}

// Test with prefix attribute
#[derive(Debug, FromEnv, PartialEq)]
#[from_env(prefix = "APP_")]
struct ServerConfigWithPrefix {
    name: String,
    listener: Listen,
}

#[test]
fn test_nested_struct_with_prefix() {
    std::env::set_var("APP_SERVERCONFIGWITHPREFIX_NAME", "my-app");
    std::env::set_var("APP_SERVERCONFIGWITHPREFIX_LISTENER_ADDRESS", "localhost");
    std::env::set_var("APP_SERVERCONFIGWITHPREFIX_LISTENER_PORT", "3000");

    let result = ServerConfigWithPrefix::from_env();
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());

    let config = result.unwrap();
    assert_eq!(config.name, "my-app");
    assert_eq!(config.listener.address, "localhost");
    assert_eq!(config.listener.port, 3000);

    // Clean up
    std::env::remove_var("APP_SERVERCONFIGWITHPREFIX_NAME");
    std::env::remove_var("APP_SERVERCONFIGWITHPREFIX_LISTENER_ADDRESS");
    std::env::remove_var("APP_SERVERCONFIGWITHPREFIX_LISTENER_PORT");
}

// Test missing required field in nested struct
#[test]
fn test_nested_struct_missing_field() {
    std::env::set_var("SERVERCONFIGREQUIRED2_DISABLED", "false");
    std::env::set_var("SERVERCONFIGREQUIRED2_LISTENER_ADDRESS", "0.0.0.0");
    // Missing PORT - this should fail
    std::env::remove_var("SERVERCONFIGREQUIRED2_LISTENER_PORT");

    let result = ServerConfigRequired2::from_env();
    assert!(result.is_err());

    std::env::remove_var("SERVERCONFIGREQUIRED2_DISABLED");
    std::env::remove_var("SERVERCONFIGREQUIRED2_LISTENER_ADDRESS");
}

#[derive(Debug, FromEnv, PartialEq)]
struct ServerConfigRequired2 {
    disabled: bool,
    listener: Listen,
}
