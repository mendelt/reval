//! Reval error types

use crate::{function, value::Value};
use displaydoc::Display as DisplayDoc;
use std::result;

/// Result type for anything Reval
pub type Result<T> = result::Result<T, Error>;

/// Anything that can go wrong in Reval should be represented here
#[derive(Debug, DisplayDoc, thiserror::Error)]
pub enum Error {
    /// An error occured serializing to a value; {0}
    ValueSerializationError(String),

    /// Tried to perform an operation on a value with an invalid type
    InvalidType,

    /// Unexpected value {0:?}, expected {1}
    UnexpectedValueType(Value, String),

    /// Unknown value {0}
    UnknownValue(String),

    /// Error executing user function {0}; {1}
    UserFunctionError(String, function::FunctionError),

    /// No user function with name {0}
    UnknownUserFunction(String),
}

impl Error {
    pub fn ser(error_msg: impl Into<String>) -> Self {
        Error::ValueSerializationError(error_msg.into())
    }
}
