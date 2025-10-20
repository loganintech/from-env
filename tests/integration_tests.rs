use from_env::{FromEnv, FromEnvTrait};

// Test 1: Simple struct from env
#[derive(Debug, FromEnv)]
struct TestSimpleStruct {
    name: String,
    age: u32,
    active: bool,
}

#[test]
fn test_simple_struct_from_env() {
    std::env::set_var("TESTSIMPLESTRUCT_NAME", "Alice");
    std::env::set_var("TESTSIMPLESTRUCT_AGE", "30");
    std::env::set_var("TESTSIMPLESTRUCT_ACTIVE", "true");

    let result = TestSimpleStruct::from_env();
    assert!(result.is_ok());
    let simple = result.unwrap();
    assert_eq!(simple.name, "Alice");
    assert_eq!(simple.age, 30);
    assert_eq!(simple.active, true);

    std::env::remove_var("TESTSIMPLESTRUCT_NAME");
    std::env::remove_var("TESTSIMPLESTRUCT_AGE");
    std::env::remove_var("TESTSIMPLESTRUCT_ACTIVE");
}

// Test 2: Struct with prefix
#[derive(Debug, FromEnv)]
#[from_env(prefix = "APP_")]
struct TestStructWithPrefix {
    host: String,
    port: u16,
}

#[test]
fn test_struct_with_prefix() {
    std::env::set_var("APP_TESTSTRUCTWITHPREFIX_HOST", "localhost");
    std::env::set_var("APP_TESTSTRUCTWITHPREFIX_PORT", "8080");

    let result = TestStructWithPrefix::from_env();
    if let Err(e) = &result {
        eprintln!("Error: {}", e);
    }
    assert!(result.is_ok());
    let config = result.unwrap();
    assert_eq!(config.host, "localhost");
    assert_eq!(config.port, 8080);

    std::env::remove_var("APP_TESTSTRUCTWITHPREFIX_HOST");
    std::env::remove_var("APP_TESTSTRUCTWITHPREFIX_PORT");
}

// Test 3: Word separator
#[derive(Debug, FromEnv)]
#[from_env(word_separator = "_")]
struct TestAppleBerry {
    color: String,
    sweet: bool,
}

#[test]
fn test_word_separator() {
    std::env::set_var("TEST_APPLE_BERRY_COLOR", "red");
    std::env::set_var("TEST_APPLE_BERRY_SWEET", "1");

    let result = TestAppleBerry::from_env();
    assert!(result.is_ok());
    let berry = result.unwrap();
    assert_eq!(berry.color, "red");
    assert_eq!(berry.sweet, true);

    std::env::remove_var("TEST_APPLE_BERRY_COLOR");
    std::env::remove_var("TEST_APPLE_BERRY_SWEET");
}

// Test 4: Load from env
#[derive(Debug, FromEnv)]
struct TestLoadStruct {
    name: String,
    age: u32,
    active: bool,
}

#[test]
fn test_load_from_env() {
    std::env::set_var("TESTLOADSTRUCT_NAME", "Bob");
    std::env::set_var("TESTLOADSTRUCT_AGE", "25");
    std::env::set_var("TESTLOADSTRUCT_ACTIVE", "false");

    let mut simple = TestLoadStruct {
        name: "Old Name".to_string(),
        age: 0,
        active: true,
    };

    let result = simple.load_from_env();
    assert!(result.is_ok());
    assert_eq!(simple.name, "Bob");
    assert_eq!(simple.age, 25);
    assert_eq!(simple.active, false);

    std::env::remove_var("TESTLOADSTRUCT_NAME");
    std::env::remove_var("TESTLOADSTRUCT_AGE");
    std::env::remove_var("TESTLOADSTRUCT_ACTIVE");
}

// Test 5: Missing variable
#[derive(Debug, FromEnv)]
struct TestMissingVar {
    name: String,
    age: u32,
    active: bool,
}

#[test]
fn test_missing_variable() {
    std::env::remove_var("TESTMISSINGVAR_NAME");
    std::env::remove_var("TESTMISSINGVAR_AGE");
    std::env::remove_var("TESTMISSINGVAR_ACTIVE");

    let result = TestMissingVar::from_env();
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("TESTMISSINGVAR_NAME"));
}

// Test 6: Parse error
#[derive(Debug, FromEnv)]
struct TestParseError {
    name: String,
    age: u32,
    active: bool,
}

#[test]
fn test_parse_error() {
    std::env::set_var("TESTPARSEERROR_NAME", "Alice");
    std::env::set_var("TESTPARSEERROR_AGE", "not_a_number");
    std::env::set_var("TESTPARSEERROR_ACTIVE", "true");

    let result = TestParseError::from_env();
    assert!(result.is_err());

    std::env::remove_var("TESTPARSEERROR_NAME");
    std::env::remove_var("TESTPARSEERROR_AGE");
    std::env::remove_var("TESTPARSEERROR_ACTIVE");
}

// Test 7: Bool parsing
#[derive(Debug, FromEnv)]
struct TestBoolParsing {
    name: String,
    age: u32,
    active: bool,
}

#[test]
fn test_bool_parsing() {
    std::env::set_var("TESTBOOLPARSING_NAME", "Test");
    std::env::set_var("TESTBOOLPARSING_AGE", "1");

    // Test "true"
    std::env::set_var("TESTBOOLPARSING_ACTIVE", "true");
    let result = TestBoolParsing::from_env();
    assert!(result.is_ok());
    assert_eq!(result.unwrap().active, true);

    // Test "false"
    std::env::set_var("TESTBOOLPARSING_ACTIVE", "false");
    let result = TestBoolParsing::from_env();
    assert!(result.is_ok());
    assert_eq!(result.unwrap().active, false);

    // Test "1"
    std::env::set_var("TESTBOOLPARSING_ACTIVE", "1");
    let result = TestBoolParsing::from_env();
    assert!(result.is_ok());
    assert_eq!(result.unwrap().active, true);

    // Test "0"
    std::env::set_var("TESTBOOLPARSING_ACTIVE", "0");
    let result = TestBoolParsing::from_env();
    assert!(result.is_ok());
    assert_eq!(result.unwrap().active, false);

    std::env::remove_var("TESTBOOLPARSING_NAME");
    std::env::remove_var("TESTBOOLPARSING_AGE");
    std::env::remove_var("TESTBOOLPARSING_ACTIVE");
}

// Test 8: All primitives
#[derive(Debug, FromEnv)]
struct TestAllPrimitives {
    val_i8: i8,
    val_i16: i16,
    val_i32: i32,
    val_i64: i64,
    val_i128: i128,
    val_isize: isize,
    val_u8: u8,
    val_u16: u16,
    val_u32: u32,
    val_u64: u64,
    val_u128: u128,
    val_usize: usize,
    val_f32: f32,
    val_f64: f64,
    val_char: char,
    val_bool: bool,
    val_string: String,
}

#[test]
fn test_all_primitives() {
    std::env::set_var("TESTALLPRIMITIVES_VAL_I8", "-128");
    std::env::set_var("TESTALLPRIMITIVES_VAL_I16", "-32768");
    std::env::set_var("TESTALLPRIMITIVES_VAL_I32", "-2147483648");
    std::env::set_var("TESTALLPRIMITIVES_VAL_I64", "-9223372036854775808");
    std::env::set_var("TESTALLPRIMITIVES_VAL_I128", "-170141183460469231731687303715884105728");
    std::env::set_var("TESTALLPRIMITIVES_VAL_ISIZE", "-9223372036854775808");
    std::env::set_var("TESTALLPRIMITIVES_VAL_U8", "255");
    std::env::set_var("TESTALLPRIMITIVES_VAL_U16", "65535");
    std::env::set_var("TESTALLPRIMITIVES_VAL_U32", "4294967295");
    std::env::set_var("TESTALLPRIMITIVES_VAL_U64", "18446744073709551615");
    std::env::set_var("TESTALLPRIMITIVES_VAL_U128", "340282366920938463463374607431768211455");
    std::env::set_var("TESTALLPRIMITIVES_VAL_USIZE", "18446744073709551615");
    std::env::set_var("TESTALLPRIMITIVES_VAL_F32", "3.14");
    std::env::set_var("TESTALLPRIMITIVES_VAL_F64", "2.718281828459045");
    std::env::set_var("TESTALLPRIMITIVES_VAL_CHAR", "A");
    std::env::set_var("TESTALLPRIMITIVES_VAL_BOOL", "true");
    std::env::set_var("TESTALLPRIMITIVES_VAL_STRING", "Hello");

    let result = TestAllPrimitives::from_env();
    assert!(result.is_ok());
    let all = result.unwrap();

    assert_eq!(all.val_i8, -128);
    assert_eq!(all.val_i16, -32768);
    assert_eq!(all.val_i32, -2147483648);
    assert_eq!(all.val_i64, -9223372036854775808);
    assert_eq!(all.val_i128, -170141183460469231731687303715884105728);
    assert_eq!(all.val_u8, 255);
    assert_eq!(all.val_u16, 65535);
    assert_eq!(all.val_u32, 4294967295);
    assert_eq!(all.val_u64, 18446744073709551615);
    assert_eq!(all.val_u128, 340282366920938463463374607431768211455);
    assert_eq!(all.val_f32, 3.14);
    assert_eq!(all.val_f64, 2.718281828459045);
    assert_eq!(all.val_char, 'A');
    assert_eq!(all.val_bool, true);
    assert_eq!(all.val_string, "Hello");

    // Clean up
    std::env::remove_var("TESTALLPRIMITIVES_VAL_I8");
    std::env::remove_var("TESTALLPRIMITIVES_VAL_I16");
    std::env::remove_var("TESTALLPRIMITIVES_VAL_I32");
    std::env::remove_var("TESTALLPRIMITIVES_VAL_I64");
    std::env::remove_var("TESTALLPRIMITIVES_VAL_I128");
    std::env::remove_var("TESTALLPRIMITIVES_VAL_ISIZE");
    std::env::remove_var("TESTALLPRIMITIVES_VAL_U8");
    std::env::remove_var("TESTALLPRIMITIVES_VAL_U16");
    std::env::remove_var("TESTALLPRIMITIVES_VAL_U32");
    std::env::remove_var("TESTALLPRIMITIVES_VAL_U64");
    std::env::remove_var("TESTALLPRIMITIVES_VAL_U128");
    std::env::remove_var("TESTALLPRIMITIVES_VAL_USIZE");
    std::env::remove_var("TESTALLPRIMITIVES_VAL_F32");
    std::env::remove_var("TESTALLPRIMITIVES_VAL_F64");
    std::env::remove_var("TESTALLPRIMITIVES_VAL_CHAR");
    std::env::remove_var("TESTALLPRIMITIVES_VAL_BOOL");
    std::env::remove_var("TESTALLPRIMITIVES_VAL_STRING");
}
