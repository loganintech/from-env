# from-env

A Rust derive macro that automatically loads struct fields from environment variables, designed to work seamlessly with prost-generated protobuf structs.

## Features

- **Derive macro** for automatic environment variable loading
- **All primitive types** supported (String, bool, integers, floats, char)
- **Option<T>** for optional fields (None if env var not set)
- **Vec<T>** for lists with comma-separated values
- **Configurable prefixes** for environment variable names
- **Flexible word separation** (e.g., `APPLEBERRY` vs `APPLE_BERRY`)
- **Two loading methods**: create new instance or load into existing
- **Type-safe parsing** with descriptive error messages
- **Works with prost** and buf.gen.yaml
- **protoc plugin** for direct protoc integration

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
from-env = "0.1"
```

## Basic Usage

```rust
use from_env::{FromEnv, FromEnvTrait};

#[derive(FromEnv)]
struct Config {
    host: String,
    port: u16,
    debug: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set environment variables
    std::env::set_var("CONFIG_HOST", "localhost");
    std::env::set_var("CONFIG_PORT", "8080");
    std::env::set_var("CONFIG_DEBUG", "true");

    // Load from environment
    let config = Config::from_env()?;

    println!("Host: {}", config.host);
    println!("Port: {}", config.port);
    println!("Debug: {}", config.debug);

    Ok(())
}
```

## Environment Variable Naming

By default, environment variables are named: `{STRUCT_NAME}_{FIELD_NAME}`

For a struct `Lemon { color: String, sweet: bool }`:
- `LEMON_COLOR`
- `LEMON_SWEET`

### Custom Prefix

Add a prefix to all environment variable names:

```rust
#[derive(FromEnv)]
#[from_env(prefix = "APP_")]
struct Config {
    host: String,
    port: u16,
}
```

Expected environment variables:
- `APP_CONFIG_HOST`
- `APP_CONFIG_PORT`

### Word Separation

Control how multi-word struct names are handled:

```rust
// Default: no separation
#[derive(FromEnv)]
struct AppleBerry {
    color: String,
}
// Expects: APPLEBERRY_COLOR

// With separator
#[derive(FromEnv)]
#[from_env(word_separator = "_")]
struct AppleBerry {
    color: String,
}
// Expects: APPLE_BERRY_COLOR
```

### Combining Options

```rust
#[derive(FromEnv)]
#[from_env(prefix = "FRUIT_", word_separator = "_")]
struct AppleBerry {
    color: String,
}
// Expects: FRUIT_APPLE_BERRY_COLOR
```

## Supported Types

### Primitive Types

All primitive types that can be parsed from strings:

- **Integers**: `i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`, `u128`, `usize`
- **Floats**: `f32`, `f64`
- **Boolean**: `bool` (accepts: `true`, `false`, `1`, `0`)
- **Character**: `char` (single character only)
- **String**: `String`

### Optional Fields - `Option<T>`

Use `Option<T>` for fields that may not be present:

```rust
#[derive(FromEnv)]
struct Config {
    required_field: String,
    optional_field: Option<String>,  // None if env var not set
    optional_port: Option<u16>,
}
```

- If the environment variable is **not set**: field value is `None`
- If the environment variable **is set**: field value is `Some(parsed_value)`
- Parsing errors still propagate as errors

```bash
# With optional field set
CONFIG_REQUIRED_FIELD=value
CONFIG_OPTIONAL_FIELD=optional_value  # Some("optional_value")
CONFIG_OPTIONAL_PORT=8080              # Some(8080)

# With optional field not set
CONFIG_REQUIRED_FIELD=value
# CONFIG_OPTIONAL_FIELD not set        # None
# CONFIG_OPTIONAL_PORT not set          # None
```

### Lists - `Vec<T>`

Use `Vec<T>` for comma-separated lists:

```rust
#[derive(FromEnv)]
struct Config {
    tags: Vec<String>,
    ports: Vec<u16>,
    enabled: Vec<bool>,
}
```

- Values are comma-separated: `"value1,value2,value3"`
- Whitespace is automatically trimmed: `"value1 , value2 , value3"` → `["value1", "value2", "value3"]`
- Empty string results in empty vector: `""` → `[]`
- Single value works too: `"value"` → `["value"]`

```bash
CONFIG_TAGS="rust,protobuf,environment"  # vec!["rust", "protobuf", "environment"]
CONFIG_PORTS="8080,8081,8082"            # vec![8080, 8081, 8082]
CONFIG_ENABLED="true,false,true"         # vec![true, false, true]
CONFIG_EMPTY=""                          # vec![]
```

Supported vec types:
- `Vec<String>`, `Vec<i32>`, `Vec<u32>`, etc. (all integer types)
- `Vec<f32>`, `Vec<f64>`
- `Vec<bool>`
- `Vec<char>`

## Loading Methods

### `from_env()` - Create New Instance

Creates a new instance from environment variables:

```rust
let config = Config::from_env()?;
```

Returns `Result<Self, FromEnvError>`.

### `load_from_env()` - Load Into Existing

Updates an existing instance with values from environment:

```rust
let mut config = Config {
    host: "default".to_string(),
    port: 3000,
    debug: false,
};

config.load_from_env()?;
```

Returns `Result<(), FromEnvError>`.

## Error Handling

The library provides two error types:

### `FromEnvError`

- `MissingVariable(String)` - Environment variable not found
- `ParseError { var: String, source: ParseError }` - Failed to parse value

### `ParseError`

- `ParseInt` - Integer parsing failed
- `ParseFloat` - Float parsing failed
- `ParseBool` - Boolean parsing failed (expected `true`, `false`, `1`, or `0`)
- `ParseChar` - Char parsing failed (expected single character)
- `ParseVec` - Vector element parsing failed (includes element index)
- `InvalidUtf8` - Invalid UTF-8 in environment variable

Example error handling:

```rust
use from_env::FromEnvError;

match Config::from_env() {
    Ok(config) => println!("Loaded config: {:?}", config),
    Err(FromEnvError::MissingVariable(var)) => {
        eprintln!("Missing required environment variable: {}", var);
    }
    Err(FromEnvError::ParseError { var, source }) => {
        eprintln!("Failed to parse {}: {}", var, source);
    }
}
```

## Using with Prost and Protocol Buffers

### Option 1: Using prost-build in build.rs

```rust
// build.rs
fn main() {
    prost_build::Config::new()
        .type_attribute(".", "#[derive(from_env::FromEnv)]")
        .compile_protos(&["proto/config.proto"], &["proto/"])
        .unwrap();
}
```

### Option 2: Using buf.gen.yaml

```yaml
version: v2
plugins:
  - remote: buf.build/prost/plugins/prost:v0.4.0-1
    out: src
    opt:
      - type_attribute=.=#[derive(from_env::FromEnv)]
```

### Option 3: Using protoc-gen-from-env Plugin

Install the plugin:

```bash
cargo install --path protoc-gen-from-env
```

Use with protoc directly:

```bash
protoc --from-env_out=. \
       --from-env_opt=prefix=APP_,word_separator=_ \
       config.proto
```

The plugin generates helper documentation showing which derives should be added. This is experimental - the recommended approach is Option 1 or 2.

### Example with Generated Code

```proto
// config.proto
syntax = "proto3";

message DatabaseConfig {
  string host = 1;
  uint32 port = 2;
  string database = 3;
  bool ssl_enabled = 4;
}
```

After generation, use it like this:

```rust
use crate::proto::DatabaseConfig;
use from_env::FromEnvTrait;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Environment variables:
    // DATABASECONFIG_HOST=localhost
    // DATABASECONFIG_PORT=5432
    // DATABASECONFIG_DATABASE=mydb
    // DATABASECONFIG_SSL_ENABLED=true

    let config = DatabaseConfig::from_env()?;
    println!("Connecting to {}:{}", config.host, config.port);

    Ok(())
}
```

## Complete Example

```rust
use from_env::{FromEnv, FromEnvTrait};

#[derive(Debug, FromEnv)]
#[from_env(prefix = "MYAPP_")]
struct AppConfig {
    // Server settings
    host: String,
    port: u16,

    // Feature flags
    debug_mode: bool,
    max_connections: u32,

    // Advanced settings
    timeout_seconds: f64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // In production, these would be set by your deployment environment
    std::env::set_var("MYAPP_APPCONFIG_HOST", "0.0.0.0");
    std::env::set_var("MYAPP_APPCONFIG_PORT", "8080");
    std::env::set_var("MYAPP_APPCONFIG_DEBUG_MODE", "false");
    std::env::set_var("MYAPP_APPCONFIG_MAX_CONNECTIONS", "100");
    std::env::set_var("MYAPP_APPCONFIG_TIMEOUT_SECONDS", "30.5");

    // Load configuration from environment
    let config = AppConfig::from_env()?;

    println!("Starting server with configuration:");
    println!("{:#?}", config);

    Ok(())
}
```

## Future Extensions

The library is designed to be extensible. Future versions may support:

- Nested structs with flattening
- Custom type parsers via traits
- Custom separators for Vec (besides comma)
- Environment variable overrides via field attributes

## License

MIT OR Apache-2.0
