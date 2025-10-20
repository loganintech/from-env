pub use from_env_derive::FromEnv;

use std::num::{ParseFloatError, ParseIntError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FromEnvError {
    #[error("Environment variable '{0}' not found")]
    MissingVariable(String),

    #[error("Failed to parse environment variable '{var}': {source}")]
    ParseError {
        var: String,
        source: ParseError,
    },
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Failed to parse integer: {0}")]
    ParseInt(#[from] ParseIntError),

    #[error("Failed to parse float: {0}")]
    ParseFloat(#[from] ParseFloatError),

    #[error("Failed to parse boolean: expected 'true', 'false', '1', or '0', got '{0}'")]
    ParseBool(String),

    #[error("Failed to parse char: expected single character, got '{0}'")]
    ParseChar(String),

    #[error("Failed to parse vector element at index {index}: {error}")]
    ParseVec { index: usize, error: String },

    #[error("Invalid UTF-8 in environment variable")]
    InvalidUtf8,
}

pub trait FromEnvTrait {
    fn from_env() -> Result<Self, FromEnvError>
    where
        Self: Sized;

    fn load_from_env(&mut self) -> Result<(), FromEnvError>;
}

pub fn parse_env_var<T: std::str::FromStr>(var_name: &str) -> Result<T, FromEnvError>
where
    T::Err: Into<ParseError>,
{
    let value = std::env::var(var_name)
        .map_err(|_| FromEnvError::MissingVariable(var_name.to_string()))?;

    value.parse::<T>().map_err(|e| FromEnvError::ParseError {
        var: var_name.to_string(),
        source: e.into(),
    })
}

pub fn parse_bool(s: &str) -> Result<bool, ParseError> {
    match s.to_lowercase().as_str() {
        "true" | "1" => Ok(true),
        "false" | "0" => Ok(false),
        _ => Err(ParseError::ParseBool(s.to_string())),
    }
}

pub fn parse_char(s: &str) -> Result<char, ParseError> {
    let mut chars = s.chars();
    match (chars.next(), chars.next()) {
        (Some(c), None) => Ok(c),
        _ => Err(ParseError::ParseChar(s.to_string())),
    }
}

pub fn parse_vec_string(s: &str) -> Result<Vec<String>, ParseError> {
    if s.trim().is_empty() {
        return Ok(Vec::new());
    }
    Ok(s.split(',').map(|s| s.trim().to_string()).collect())
}

pub fn parse_vec_int<T>(s: &str) -> Result<Vec<T>, ParseError>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    if s.trim().is_empty() {
        return Ok(Vec::new());
    }
    s.split(',')
        .map(|s| s.trim())
        .enumerate()
        .map(|(i, s)| {
            s.parse::<T>().map_err(|e| ParseError::ParseVec {
                index: i,
                error: e.to_string(),
            })
        })
        .collect()
}

pub fn parse_vec_float<T>(s: &str) -> Result<Vec<T>, ParseError>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    if s.trim().is_empty() {
        return Ok(Vec::new());
    }
    s.split(',')
        .map(|s| s.trim())
        .enumerate()
        .map(|(i, s)| {
            s.parse::<T>().map_err(|e| ParseError::ParseVec {
                index: i,
                error: e.to_string(),
            })
        })
        .collect()
}

pub fn parse_vec_bool(s: &str) -> Result<Vec<bool>, ParseError> {
    if s.trim().is_empty() {
        return Ok(Vec::new());
    }
    s.split(',')
        .map(|s| s.trim())
        .enumerate()
        .map(|(i, s)| parse_bool(s).map_err(|_| ParseError::ParseVec {
            index: i,
            error: format!("expected 'true', 'false', '1', or '0', got '{}'", s),
        }))
        .collect()
}

pub fn parse_vec_char(s: &str) -> Result<Vec<char>, ParseError> {
    if s.trim().is_empty() {
        return Ok(Vec::new());
    }
    s.split(',')
        .map(|s| s.trim())
        .enumerate()
        .map(|(i, s)| parse_char(s).map_err(|_| ParseError::ParseVec {
            index: i,
            error: format!("expected single character, got '{}'", s),
        }))
        .collect()
}
