//! Reval error types

use crate::value::Value;
use std::{num::TryFromIntError, result};

/// Result type for anything Reval
pub type Result<T> = result::Result<T, Error>;

/// Anything that can go wrong in Reval should be represented here
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("User-function name {0} is not valid")]
    InvalidFunctionName(String),

    #[error("Duplicate user-function name; {0}")]
    DuplicateFunctionName(String),

    #[error("Duplicate rule name; {0}")]
    DuplicateRuleName(String),

    #[error("An error occured serializing to a value; {0}")]
    ValueSerializationError(String),

    #[error("Tried to perform an operation on a value with an invalid type")]
    InvalidType,

    #[error("Cannot cast value type {0:?} to {1}")]
    InvalidCast(Value, String),

    #[error("Numeric overflow casting int types")]
    NumericOverflow(#[from] TryFromIntError),

    #[error("Unexpected value {0:?}, expected {1}")]
    UnexpectedValueType(Value, String),

    #[error("Unknown field reference {0}")]
    UnknownRef(String),

    #[error("Unknown index {0}")]
    UnknownIndex(String),

    #[error("Error executing user function {function}; {error}")]
    UserFunctionError {
        function: String,
        error: anyhow::Error,
    },

    #[error("No user function with name {0}")]
    UnknownUserFunction(String),

    #[error("Error parsing rule from json {0}")]
    JsonParseError(#[from] serde_json::Error),
}

impl Error {
    pub fn ser(error_msg: impl Into<String>) -> Self {
        Error::ValueSerializationError(error_msg.into())
    }
}
