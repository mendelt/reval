//! Reval error types

use crate::value::Value;
use displaydoc::Display as DisplayDoc;
use std::{num::TryFromIntError, result};

/// Result type for anything Reval
pub type Result<T> = result::Result<T, Error>;

/// Anything that can go wrong in Reval should be represented here
#[derive(Debug, DisplayDoc, thiserror::Error)]
pub enum Error {
    /// User-function name {0} is not valid
    InvalidFunctionName(String),

    /// Rule has no name specified
    MissingRuleName,

    /// Duplicate user-function name; {0}
    DuplicateFunctionName(String),

    /// Duplicate rule name; {0}
    DuplicateRuleName(String),

    /// An error occured serializing to a value; {0}
    ValueSerializationError(String),

    /// Tried to perform an operation on a value with an invalid type
    InvalidType,

    /// Cannot cast value type {0:?} to {1}
    InvalidCast(Value, String),

    /// Numeric overflow casting int types
    NumericOverflow(#[from] TryFromIntError),

    /// Unexpected value {0:?}, expected {1}
    UnexpectedValueType(Value, String),

    /// Unknown field reference {0}
    UnknownRef(String),

    /// Unknown index {0}
    UnknownIndex(String),

    /// Error executing user function {function}; {error}
    UserFunctionError {
        function: String,
        error: anyhow::Error,
    },

    /// No user function with name {0}
    UnknownUserFunction(String),

    /// Error parsing rule from json {0}
    JsonParseError(#[from] serde_json::Error),

    /// Error parsing expression
    ExprParseError(String),
}

impl Error {
    pub fn ser(error_msg: impl Into<String>) -> Self {
        Error::ValueSerializationError(error_msg.into())
    }
}
