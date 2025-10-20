use from_env::{FromEnv, FromEnvTrait};

#[derive(Debug, FromEnv)]
struct ServerConfig {
    // Required fields
    host: String,
    port: u16,

    // Optional fields - will be None if env var is not set
    ssl_cert: Option<String>,
    ssl_key: Option<String>,
    max_connections: Option<u32>,
    timeout_seconds: Option<f64>,

    // Vector fields - comma-separated values
    allowed_hosts: Vec<String>,
    trusted_proxies: Vec<String>,
    cors_origins: Vec<String>,
}

#[derive(Debug, FromEnv)]
#[from_env(prefix = "APP_")]
struct FeatureConfig {
    // Simple flags
    enabled: bool,

    // Optional feature toggles
    experimental_mode: Option<bool>,
    debug_level: Option<u8>,

    // Lists of enabled features
    enabled_modules: Vec<String>,
    admin_users: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Option and Vec Example ===\n");

    // Example 1: Server config with optional SSL
    println!("1. Server configuration with optional SSL:");
    std::env::set_var("SERVERCONFIG_HOST", "0.0.0.0");
    std::env::set_var("SERVERCONFIG_PORT", "8080");
    // SSL fields not set - will be None
    std::env::set_var("SERVERCONFIG_MAX_CONNECTIONS", "1000");
    std::env::set_var("SERVERCONFIG_ALLOWED_HOSTS", "localhost,127.0.0.1,example.com");
    std::env::set_var("SERVERCONFIG_TRUSTED_PROXIES", "10.0.0.1");
    std::env::set_var("SERVERCONFIG_CORS_ORIGINS", "");  // Empty list

    let server = ServerConfig::from_env()?;
    println!("   Host: {}", server.host);
    println!("   Port: {}", server.port);
    println!("   SSL Cert: {:?}", server.ssl_cert);
    println!("   SSL Key: {:?}", server.ssl_key);
    println!("   Max Connections: {:?}", server.max_connections);
    println!("   Allowed Hosts: {:?}", server.allowed_hosts);
    println!("   Trusted Proxies: {:?}", server.trusted_proxies);
    println!("   CORS Origins: {:?} (empty vec)\n", server.cors_origins);

    // Example 2: Same config with SSL enabled
    println!("2. Same server with SSL enabled:");
    std::env::set_var("SERVERCONFIG_SSL_CERT", "/path/to/cert.pem");
    std::env::set_var("SERVERCONFIG_SSL_KEY", "/path/to/key.pem");
    std::env::set_var("SERVERCONFIG_TIMEOUT_SECONDS", "30.5");

    let server_with_ssl = ServerConfig::from_env()?;
    println!("   Host: {}", server_with_ssl.host);
    println!("   Port: {}", server_with_ssl.port);
    println!("   SSL Cert: {:?}", server_with_ssl.ssl_cert);
    println!("   SSL Key: {:?}", server_with_ssl.ssl_key);
    println!("   Timeout: {:?}\n", server_with_ssl.timeout_seconds);

    // Example 3: Feature config with prefix
    println!("3. Feature configuration with prefix:");
    std::env::set_var("APP_FEATURECONFIG_ENABLED", "true");
    std::env::set_var("APP_FEATURECONFIG_EXPERIMENTAL_MODE", "true");
    std::env::set_var("APP_FEATURECONFIG_DEBUG_LEVEL", "3");
    std::env::set_var("APP_FEATURECONFIG_ENABLED_MODULES", "auth,api,admin,analytics");
    std::env::set_var("APP_FEATURECONFIG_ADMIN_USERS", "alice,bob,charlie");

    let features = FeatureConfig::from_env()?;
    println!("   Enabled: {}", features.enabled);
    println!("   Experimental Mode: {:?}", features.experimental_mode);
    println!("   Debug Level: {:?}", features.debug_level);
    println!("   Enabled Modules: {:?}", features.enabled_modules);
    println!("   Admin Users: {:?}\n", features.admin_users);

    // Example 4: Updating existing config
    println!("4. Updating existing configuration:");
    let mut config = ServerConfig {
        host: "old-host".to_string(),
        port: 3000,
        ssl_cert: Some("old-cert".to_string()),
        ssl_key: Some("old-key".to_string()),
        max_connections: Some(100),
        timeout_seconds: None,
        allowed_hosts: vec!["old-host".to_string()],
        trusted_proxies: vec![],
        cors_origins: vec![],
    };

    println!("   Before: {:#?}", config);

    // Remove SSL cert to test Option becomes None
    std::env::remove_var("SERVERCONFIG_SSL_CERT");

    config.load_from_env()?;
    println!("   After load_from_env: {:#?}\n", config);

    // Example 5: Demonstrating empty vs missing
    println!("5. Empty string vs missing env var:");
    std::env::set_var("SERVERCONFIG_HOST", "test");
    std::env::set_var("SERVERCONFIG_PORT", "8080");

    // Empty vec (env var exists but is empty)
    std::env::set_var("SERVERCONFIG_ALLOWED_HOSTS", "");

    // Optional present with empty value (this is still Some)
    std::env::set_var("SERVERCONFIG_SSL_CERT", "");

    // Optional not set (will be None)
    std::env::remove_var("SERVERCONFIG_SSL_KEY");

    std::env::set_var("SERVERCONFIG_TRUSTED_PROXIES", "");
    std::env::set_var("SERVERCONFIG_CORS_ORIGINS", "");

    let config = ServerConfig::from_env()?;
    println!("   Allowed Hosts (empty string): {:?}", config.allowed_hosts);
    println!("   SSL Cert (empty string): {:?}", config.ssl_cert);
    println!("   SSL Key (not set): {:?}\n", config.ssl_key);

    println!("=== Key Takeaways ===");
    println!("- Option<T>: None if env var not set, Some(parsed_value) if set");
    println!("- Vec<T>: Parse comma-separated values, empty vec if empty string");
    println!("- Whitespace around commas is automatically trimmed");
    println!("- Combine with prefix and word_separator for flexible naming");

    Ok(())
}
