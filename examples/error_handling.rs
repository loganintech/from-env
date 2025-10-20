use from_env::{FromEnv, FromEnvError, FromEnvTrait};

#[derive(Debug, FromEnv)]
struct Config {
    host: String,
    port: u16,
    timeout: f32,
    enabled: bool,
}

fn main() {
    println!("=== Error Handling Example ===\n");

    // Example 1: Missing variable
    println!("1. Missing environment variable:");
    std::env::remove_var("CONFIG_HOST");
    std::env::remove_var("CONFIG_PORT");
    std::env::remove_var("CONFIG_TIMEOUT");
    std::env::remove_var("CONFIG_ENABLED");

    match Config::from_env() {
        Ok(_) => println!("   Unexpected success"),
        Err(FromEnvError::MissingVariable(var)) => {
            println!("   ✓ Expected error: Missing variable '{}'", var);
        }
        Err(e) => println!("   Unexpected error: {}", e),
    }

    // Example 2: Parse error - invalid integer
    println!("\n2. Parse error - invalid integer:");
    std::env::set_var("CONFIG_HOST", "localhost");
    std::env::set_var("CONFIG_PORT", "not_a_number");
    std::env::set_var("CONFIG_TIMEOUT", "30.0");
    std::env::set_var("CONFIG_ENABLED", "true");

    match Config::from_env() {
        Ok(_) => println!("   Unexpected success"),
        Err(FromEnvError::ParseError { var, source }) => {
            println!("   ✓ Expected error: Failed to parse '{}': {}", var, source);
        }
        Err(e) => println!("   Unexpected error: {}", e),
    }

    // Example 3: Parse error - invalid boolean
    println!("\n3. Parse error - invalid boolean:");
    std::env::set_var("CONFIG_PORT", "8080");
    std::env::set_var("CONFIG_ENABLED", "maybe");

    match Config::from_env() {
        Ok(_) => println!("   Unexpected success"),
        Err(FromEnvError::ParseError { var, source }) => {
            println!("   ✓ Expected error: Failed to parse '{}': {}", var, source);
        }
        Err(e) => println!("   Unexpected error: {}", e),
    }

    // Example 4: Valid boolean values
    println!("\n4. Valid boolean values:");
    let valid_bools = vec!["true", "false", "1", "0"];
    for val in valid_bools {
        std::env::set_var("CONFIG_ENABLED", val);
        match Config::from_env() {
            Ok(config) => println!("   ✓ '{}' parsed as: {}", val, config.enabled),
            Err(e) => println!("   ✗ Failed to parse '{}': {}", val, e),
        }
    }

    // Example 5: Successful load
    println!("\n5. Successful configuration load:");
    std::env::set_var("CONFIG_HOST", "api.example.com");
    std::env::set_var("CONFIG_PORT", "443");
    std::env::set_var("CONFIG_TIMEOUT", "60.5");
    std::env::set_var("CONFIG_ENABLED", "true");

    match Config::from_env() {
        Ok(config) => {
            println!("   ✓ Configuration loaded successfully:");
            println!("     Host: {}", config.host);
            println!("     Port: {}", config.port);
            println!("     Timeout: {}", config.timeout);
            println!("     Enabled: {}", config.enabled);
        }
        Err(e) => println!("   ✗ Unexpected error: {}", e),
    }

    // Example 6: Using Result with ?
    println!("\n6. Using Result with ? operator:");
    fn load_config() -> Result<Config, FromEnvError> {
        let config = Config::from_env()?;
        println!("   Config loaded in function");
        Ok(config)
    }

    match load_config() {
        Ok(_) => println!("   ✓ Successfully loaded via function"),
        Err(e) => println!("   ✗ Error: {}", e),
    }
}
