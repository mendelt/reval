use std::result;

use displaydoc::Display as DisplayDoc;

/// Result type for anything reval
pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, DisplayDoc, thiserror::Error)]
pub enum Error {
    /// An error occured serializing to a value; {0}
    ValueSerializationError(String),

    /// Tried to perform an operation on a value with an invalid type
    InvalidType,

    /// Unknown value {0}
    UnknownValue(String),
}

impl Error {
    pub fn ser(error_msg: impl Into<String>) -> Self {
        Error::ValueSerializationError(error_msg.into())
    }
}
