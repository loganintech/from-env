use from_env::{FromEnv, FromEnvTrait};

#[derive(Debug, FromEnv)]
struct Config {
    host: String,
    port: u16,
    debug: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Basic FromEnv Example ===\n");

    // Set environment variables
    println!("Setting environment variables:");
    std::env::set_var("CONFIG_HOST", "localhost");
    std::env::set_var("CONFIG_PORT", "8080");
    std::env::set_var("CONFIG_DEBUG", "true");
    println!("  CONFIG_HOST=localhost");
    println!("  CONFIG_PORT=8080");
    println!("  CONFIG_DEBUG=true\n");

    // Load from environment
    println!("Loading configuration from environment...");
    let config = Config::from_env()?;

    println!("\nLoaded configuration:");
    println!("  Host: {}", config.host);
    println!("  Port: {}", config.port);
    println!("  Debug: {}", config.debug);

    Ok(())
}
