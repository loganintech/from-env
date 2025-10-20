use from_env::{FromEnv, FromEnvTrait};

#[derive(Debug, FromEnv)]
#[from_env(prefix = "APP_")]
struct ServerConfig {
    host: String,
    port: u16,
    ssl_enabled: bool,
}

#[derive(Debug, FromEnv)]
#[from_env(word_separator = "_")]
struct AppleBerry {
    color: String,
    sweetness: f32,
}

#[derive(Debug, FromEnv)]
#[from_env(prefix = "FRUIT_", word_separator = "_")]
struct OrangePeach {
    color: String,
    ripeness: u8,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Advanced FromEnv Example ===\n");

    // Example 1: With prefix
    println!("1. Configuration with prefix:");
    std::env::set_var("APP_SERVERCONFIG_HOST", "0.0.0.0");
    std::env::set_var("APP_SERVERCONFIG_PORT", "443");
    std::env::set_var("APP_SERVERCONFIG_SSL_ENABLED", "true");
    println!("   Environment variables:");
    println!("     APP_SERVERCONFIG_HOST=0.0.0.0");
    println!("     APP_SERVERCONFIG_PORT=443");
    println!("     APP_SERVERCONFIG_SSL_ENABLED=true");

    let server = ServerConfig::from_env()?;
    println!("   Loaded: {:#?}\n", server);

    // Example 2: With word separator
    println!("2. Multi-word struct with separator:");
    std::env::set_var("APPLE_BERRY_COLOR", "red");
    std::env::set_var("APPLE_BERRY_SWEETNESS", "8.5");
    println!("   Environment variables:");
    println!("     APPLE_BERRY_COLOR=red");
    println!("     APPLE_BERRY_SWEETNESS=8.5");

    let berry = AppleBerry::from_env()?;
    println!("   Loaded: {:#?}\n", berry);

    // Example 3: Both prefix and word separator
    println!("3. Both prefix and word separator:");
    std::env::set_var("FRUIT_ORANGE_PEACH_COLOR", "orange");
    std::env::set_var("FRUIT_ORANGE_PEACH_RIPENESS", "95");
    println!("   Environment variables:");
    println!("     FRUIT_ORANGE_PEACH_COLOR=orange");
    println!("     FRUIT_ORANGE_PEACH_RIPENESS=95");

    let fruit = OrangePeach::from_env()?;
    println!("   Loaded: {:#?}\n", fruit);

    // Example 4: Using load_from_env
    println!("4. Loading into existing instance:");
    let mut berry = AppleBerry {
        color: "green".to_string(),
        sweetness: 3.0,
    };
    println!("   Before: {:#?}", berry);

    std::env::set_var("APPLE_BERRY_COLOR", "purple");
    std::env::set_var("APPLE_BERRY_SWEETNESS", "9.2");
    berry.load_from_env()?;
    println!("   After:  {:#?}\n", berry);

    Ok(())
}
