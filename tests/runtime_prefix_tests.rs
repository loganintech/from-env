use from_env::{FromEnv, FromEnvTrait};

#[derive(Debug, FromEnv)]
struct ServiceConfig {
    host: String,
    port: u16,
    debug: bool,
}

#[test]
fn test_from_env_with_prefix_service_a() {
    std::env::set_var("SERVICE_A_SERVICECONFIG_HOST", "localhost");
    std::env::set_var("SERVICE_A_SERVICECONFIG_PORT", "8080");
    std::env::set_var("SERVICE_A_SERVICECONFIG_DEBUG", "true");

    let config = ServiceConfig::from_env_with_prefix("SERVICE_A_").unwrap();
    assert_eq!(config.host, "localhost");
    assert_eq!(config.port, 8080);
    assert_eq!(config.debug, true);

    std::env::remove_var("SERVICE_A_SERVICECONFIG_HOST");
    std::env::remove_var("SERVICE_A_SERVICECONFIG_PORT");
    std::env::remove_var("SERVICE_A_SERVICECONFIG_DEBUG");
}

#[test]
fn test_from_env_with_prefix_service_b() {
    std::env::set_var("SERVICE_B_SERVICECONFIG_HOST", "example.com");
    std::env::set_var("SERVICE_B_SERVICECONFIG_PORT", "9000");
    std::env::set_var("SERVICE_B_SERVICECONFIG_DEBUG", "false");

    let config = ServiceConfig::from_env_with_prefix("SERVICE_B_").unwrap();
    assert_eq!(config.host, "example.com");
    assert_eq!(config.port, 9000);
    assert_eq!(config.debug, false);

    std::env::remove_var("SERVICE_B_SERVICECONFIG_HOST");
    std::env::remove_var("SERVICE_B_SERVICECONFIG_PORT");
    std::env::remove_var("SERVICE_B_SERVICECONFIG_DEBUG");
}

#[test]
fn test_from_env_with_prefix_empty_prefix() {
    std::env::set_var("SERVICECONFIG_HOST", "api.example.com");
    std::env::set_var("SERVICECONFIG_PORT", "443");
    std::env::set_var("SERVICECONFIG_DEBUG", "1");

    let config = ServiceConfig::from_env_with_prefix("").unwrap();
    assert_eq!(config.host, "api.example.com");
    assert_eq!(config.port, 443);
    assert_eq!(config.debug, true);

    std::env::remove_var("SERVICECONFIG_HOST");
    std::env::remove_var("SERVICECONFIG_PORT");
    std::env::remove_var("SERVICECONFIG_DEBUG");
}

#[test]
fn test_load_from_env_with_prefix() {
    std::env::set_var("APP_SERVICECONFIG_HOST", "updated.com");
    std::env::set_var("APP_SERVICECONFIG_PORT", "3000");
    std::env::set_var("APP_SERVICECONFIG_DEBUG", "true");

    let mut config = ServiceConfig {
        host: "old.com".to_string(),
        port: 1000,
        debug: false,
    };

    config.load_from_env_with_prefix("APP_").unwrap();
    assert_eq!(config.host, "updated.com");
    assert_eq!(config.port, 3000);
    assert_eq!(config.debug, true);

    std::env::remove_var("APP_SERVICECONFIG_HOST");
    std::env::remove_var("APP_SERVICECONFIG_PORT");
    std::env::remove_var("APP_SERVICECONFIG_DEBUG");
}

#[test]
fn test_from_env_with_prefix_missing_variable() {
    let result = ServiceConfig::from_env_with_prefix("MISSING_");
    assert!(result.is_err());

    if let Err(e) = result {
        let error_msg = format!("{}", e);
        assert!(error_msg.contains("not found"));
    }
}

#[derive(Debug, FromEnv)]
struct ConfigWithOptions {
    required: String,
    optional: Option<String>,
    optional_num: Option<i32>,
}

#[test]
fn test_from_env_with_prefix_with_options() {
    std::env::set_var("PREFIX_CONFIGWITHOPTIONS_REQUIRED", "value");
    std::env::set_var("PREFIX_CONFIGWITHOPTIONS_OPTIONAL", "present");
    // optional_num is not set

    let config = ConfigWithOptions::from_env_with_prefix("PREFIX_").unwrap();
    assert_eq!(config.required, "value");
    assert_eq!(config.optional, Some("present".to_string()));
    assert_eq!(config.optional_num, None);

    std::env::remove_var("PREFIX_CONFIGWITHOPTIONS_REQUIRED");
    std::env::remove_var("PREFIX_CONFIGWITHOPTIONS_OPTIONAL");
}

#[derive(Debug, FromEnv)]
struct ConfigWithVecs {
    tags: Vec<String>,
    ports: Vec<u16>,
}

#[test]
fn test_from_env_with_prefix_with_vecs() {
    std::env::set_var("MY_CONFIGWITHVECS_TAGS", "tag1, tag2, tag3");
    std::env::set_var("MY_CONFIGWITHVECS_PORTS", "8080, 8081, 8082");

    let config = ConfigWithVecs::from_env_with_prefix("MY_").unwrap();
    assert_eq!(config.tags, vec!["tag1", "tag2", "tag3"]);
    assert_eq!(config.ports, vec![8080, 8081, 8082]);

    std::env::remove_var("MY_CONFIGWITHVECS_TAGS");
    std::env::remove_var("MY_CONFIGWITHVECS_PORTS");
}
