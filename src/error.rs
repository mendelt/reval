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

    #[error("Value {0} out of bounds for function {1}")]
    ValueOutOfBounds(Value, String),

    #[error("Invalid symbol; {0}")]
    InvalidSymbol(String),
}

impl Error {
    pub fn ser(error_msg: impl Into<String>) -> Self {
        Error::ValueSerializationError(error_msg.into())
    }

    /// Construct InvalidCast Error
    pub fn invalid_cast(value: Value, to: &str) -> Self {
        Error::InvalidCast(value, to.to_string())
    }

    /// Construct UnexpectedValueType Error
    pub fn unexpected_val_type(value: Value, expect: &str) -> Self {
        Error::UnexpectedValueType(value, expect.to_string())
    }

    pub fn value_out_of_bounds(value: Value, function: &str) -> Self {
        Error::ValueOutOfBounds(value, function.to_string())
    }
}
