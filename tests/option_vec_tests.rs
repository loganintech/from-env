use from_env::{FromEnv, FromEnvTrait};

// Test 1: Option with value
#[derive(Debug, FromEnv)]
struct TestOptionWithValue {
    required: String,
    optional_string: Option<String>,
    optional_int: Option<i32>,
    optional_bool: Option<bool>,
}

#[test]
fn test_option_with_value() {
    std::env::set_var("TESTOPTIONWITHVALUE_REQUIRED", "test");
    std::env::set_var("TESTOPTIONWITHVALUE_OPTIONAL_STRING", "value");
    std::env::set_var("TESTOPTIONWITHVALUE_OPTIONAL_INT", "42");
    std::env::set_var("TESTOPTIONWITHVALUE_OPTIONAL_BOOL", "true");

    let config = TestOptionWithValue::from_env().unwrap();
    assert_eq!(config.required, "test");
    assert_eq!(config.optional_string, Some("value".to_string()));
    assert_eq!(config.optional_int, Some(42));
    assert_eq!(config.optional_bool, Some(true));

    std::env::remove_var("TESTOPTIONWITHVALUE_REQUIRED");
    std::env::remove_var("TESTOPTIONWITHVALUE_OPTIONAL_STRING");
    std::env::remove_var("TESTOPTIONWITHVALUE_OPTIONAL_INT");
    std::env::remove_var("TESTOPTIONWITHVALUE_OPTIONAL_BOOL");
}

// Test 2: Option without value
#[derive(Debug, FromEnv)]
struct TestOptionWithoutValue {
    required: String,
    optional_string: Option<String>,
    optional_int: Option<i32>,
    optional_bool: Option<bool>,
}

#[test]
fn test_option_without_value() {
    std::env::set_var("TESTOPTIONWITHOUTVALUE_REQUIRED", "test");
    std::env::remove_var("TESTOPTIONWITHOUTVALUE_OPTIONAL_STRING");
    std::env::remove_var("TESTOPTIONWITHOUTVALUE_OPTIONAL_INT");
    std::env::remove_var("TESTOPTIONWITHOUTVALUE_OPTIONAL_BOOL");

    let config = TestOptionWithoutValue::from_env().unwrap();
    assert_eq!(config.required, "test");
    assert_eq!(config.optional_string, None);
    assert_eq!(config.optional_int, None);
    assert_eq!(config.optional_bool, None);

    std::env::remove_var("TESTOPTIONWITHOUTVALUE_REQUIRED");
}

// Test 3: Vec strings
#[derive(Debug, FromEnv)]
struct TestVecStrings {
    tags: Vec<String>,
    scores: Vec<i32>,
    flags: Vec<bool>,
    ratios: Vec<f64>,
}

#[test]
fn test_vec_strings() {
    std::env::set_var("TESTVECSTRINGS_TAGS", "rust,cargo,prost");
    std::env::set_var("TESTVECSTRINGS_SCORES", "10,20,30");
    std::env::set_var("TESTVECSTRINGS_FLAGS", "true,false,true");
    std::env::set_var("TESTVECSTRINGS_RATIOS", "1.5,2.7,3.14");

    let config = TestVecStrings::from_env().unwrap();
    assert_eq!(config.tags, vec!["rust", "cargo", "prost"]);
    assert_eq!(config.scores, vec![10, 20, 30]);
    assert_eq!(config.flags, vec![true, false, true]);
    assert_eq!(config.ratios, vec![1.5, 2.7, 3.14]);

    std::env::remove_var("TESTVECSTRINGS_TAGS");
    std::env::remove_var("TESTVECSTRINGS_SCORES");
    std::env::remove_var("TESTVECSTRINGS_FLAGS");
    std::env::remove_var("TESTVECSTRINGS_RATIOS");
}

// Test 4: Vec empty
#[derive(Debug, FromEnv)]
struct TestVecEmpty {
    tags: Vec<String>,
    scores: Vec<i32>,
    flags: Vec<bool>,
    ratios: Vec<f64>,
}

#[test]
fn test_vec_empty() {
    std::env::set_var("TESTVECEMPTY_TAGS", "");
    std::env::set_var("TESTVECEMPTY_SCORES", "");
    std::env::set_var("TESTVECEMPTY_FLAGS", "");
    std::env::set_var("TESTVECEMPTY_RATIOS", "");

    let config = TestVecEmpty::from_env().unwrap();
    assert_eq!(config.tags, Vec::<String>::new());
    assert_eq!(config.scores, Vec::<i32>::new());
    assert_eq!(config.flags, Vec::<bool>::new());
    assert_eq!(config.ratios, Vec::<f64>::new());

    std::env::remove_var("TESTVECEMPTY_TAGS");
    std::env::remove_var("TESTVECEMPTY_SCORES");
    std::env::remove_var("TESTVECEMPTY_FLAGS");
    std::env::remove_var("TESTVECEMPTY_RATIOS");
}

// Test 5: Vec single value
#[derive(Debug, FromEnv)]
struct TestVecSingle {
    tags: Vec<String>,
    scores: Vec<i32>,
    flags: Vec<bool>,
    ratios: Vec<f64>,
}

#[test]
fn test_vec_single_value() {
    std::env::set_var("TESTVECSINGLE_TAGS", "single");
    std::env::set_var("TESTVECSINGLE_SCORES", "42");
    std::env::set_var("TESTVECSINGLE_FLAGS", "true");
    std::env::set_var("TESTVECSINGLE_RATIOS", "3.14");

    let config = TestVecSingle::from_env().unwrap();
    assert_eq!(config.tags, vec!["single"]);
    assert_eq!(config.scores, vec![42]);
    assert_eq!(config.flags, vec![true]);
    assert_eq!(config.ratios, vec![3.14]);

    std::env::remove_var("TESTVECSINGLE_TAGS");
    std::env::remove_var("TESTVECSINGLE_SCORES");
    std::env::remove_var("TESTVECSINGLE_FLAGS");
    std::env::remove_var("TESTVECSINGLE_RATIOS");
}

// Test 6: Vec with spaces
#[derive(Debug, FromEnv)]
struct TestVecSpaces {
    tags: Vec<String>,
    scores: Vec<i32>,
    flags: Vec<bool>,
    ratios: Vec<f64>,
}

#[test]
fn test_vec_with_spaces() {
    std::env::set_var("TESTVECSPACES_TAGS", "  rust  ,  cargo  ,  prost  ");
    std::env::set_var("TESTVECSPACES_SCORES", " 10 , 20 , 30 ");
    std::env::set_var("TESTVECSPACES_FLAGS", " true , false , true ");
    std::env::set_var("TESTVECSPACES_RATIOS", " 1.5 , 2.7 , 3.14 ");

    let config = TestVecSpaces::from_env().unwrap();
    assert_eq!(config.tags, vec!["rust", "cargo", "prost"]);
    assert_eq!(config.scores, vec![10, 20, 30]);
    assert_eq!(config.flags, vec![true, false, true]);
    assert_eq!(config.ratios, vec![1.5, 2.7, 3.14]);

    std::env::remove_var("TESTVECSPACES_TAGS");
    std::env::remove_var("TESTVECSPACES_SCORES");
    std::env::remove_var("TESTVECSPACES_FLAGS");
    std::env::remove_var("TESTVECSPACES_RATIOS");
}

// Test 7: Mixed config
#[derive(Debug, FromEnv)]
struct TestMixedConfig {
    name: String,
    optional_port: Option<u16>,
    hosts: Vec<String>,
    optional_timeout: Option<f32>,
}

#[test]
fn test_mixed_config() {
    std::env::set_var("TESTMIXEDCONFIG_NAME", "my-app");
    std::env::set_var("TESTMIXEDCONFIG_OPTIONAL_PORT", "8080");
    std::env::set_var("TESTMIXEDCONFIG_HOSTS", "localhost,127.0.0.1");
    std::env::remove_var("TESTMIXEDCONFIG_OPTIONAL_TIMEOUT");

    let config = TestMixedConfig::from_env().unwrap();
    assert_eq!(config.name, "my-app");
    assert_eq!(config.optional_port, Some(8080));
    assert_eq!(config.hosts, vec!["localhost", "127.0.0.1"]);
    assert_eq!(config.optional_timeout, None);

    std::env::remove_var("TESTMIXEDCONFIG_NAME");
    std::env::remove_var("TESTMIXEDCONFIG_OPTIONAL_PORT");
    std::env::remove_var("TESTMIXEDCONFIG_HOSTS");
}

// Test 8: Load from env with option
#[derive(Debug, FromEnv)]
struct TestLoadOption {
    required: String,
    optional_string: Option<String>,
    optional_int: Option<i32>,
    optional_bool: Option<bool>,
}

#[test]
fn test_load_from_env_with_option() {
    std::env::set_var("TESTLOADOPTION_REQUIRED", "updated");
    std::env::set_var("TESTLOADOPTION_OPTIONAL_STRING", "new_value");
    std::env::remove_var("TESTLOADOPTION_OPTIONAL_INT");
    std::env::set_var("TESTLOADOPTION_OPTIONAL_BOOL", "false");

    let mut config = TestLoadOption {
        required: "old".to_string(),
        optional_string: None,
        optional_int: Some(99),
        optional_bool: None,
    };

    config.load_from_env().unwrap();
    assert_eq!(config.required, "updated");
    assert_eq!(config.optional_string, Some("new_value".to_string()));
    assert_eq!(config.optional_int, None);
    assert_eq!(config.optional_bool, Some(false));

    std::env::remove_var("TESTLOADOPTION_REQUIRED");
    std::env::remove_var("TESTLOADOPTION_OPTIONAL_STRING");
    std::env::remove_var("TESTLOADOPTION_OPTIONAL_BOOL");
}

// Test 9: Vec parse error
#[derive(Debug, FromEnv)]
struct TestVecParseError {
    tags: Vec<String>,
    scores: Vec<i32>,
    flags: Vec<bool>,
    ratios: Vec<f64>,
}

#[test]
fn test_vec_parse_error() {
    std::env::set_var("TESTVECPARSEERROR_TAGS", "a,b,c");
    std::env::set_var("TESTVECPARSEERROR_SCORES", "10,not_a_number,30");
    std::env::set_var("TESTVECPARSEERROR_FLAGS", "true,false");
    std::env::set_var("TESTVECPARSEERROR_RATIOS", "1.5,2.7");

    let result = TestVecParseError::from_env();
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("TESTVECPARSEERROR_SCORES"));

    std::env::remove_var("TESTVECPARSEERROR_TAGS");
    std::env::remove_var("TESTVECPARSEERROR_SCORES");
    std::env::remove_var("TESTVECPARSEERROR_FLAGS");
    std::env::remove_var("TESTVECPARSEERROR_RATIOS");
}

// Test 10: Option parse error
#[derive(Debug, FromEnv)]
struct TestOptionParseError {
    required: String,
    optional_string: Option<String>,
    optional_int: Option<i32>,
    optional_bool: Option<bool>,
}

#[test]
fn test_option_parse_error() {
    std::env::set_var("TESTOPTIONPARSEERROR_REQUIRED", "test");
    std::env::set_var("TESTOPTIONPARSEERROR_OPTIONAL_STRING", "value");
    std::env::set_var("TESTOPTIONPARSEERROR_OPTIONAL_INT", "not_a_number");
    std::env::set_var("TESTOPTIONPARSEERROR_OPTIONAL_BOOL", "true");

    let result = TestOptionParseError::from_env();
    assert!(result.is_err());

    std::env::remove_var("TESTOPTIONPARSEERROR_REQUIRED");
    std::env::remove_var("TESTOPTIONPARSEERROR_OPTIONAL_STRING");
    std::env::remove_var("TESTOPTIONPARSEERROR_OPTIONAL_INT");
    std::env::remove_var("TESTOPTIONPARSEERROR_OPTIONAL_BOOL");
}
