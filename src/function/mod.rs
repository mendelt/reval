//! User functions
mod context;
mod user_functions;

use crate::{error::Error, value::Value};
use async_trait::async_trait;
use displaydoc::Display as DisplayDoc;
use std::error;
use std::result;

pub use context::FunctionContext;
pub use user_functions::UserFunctions;

/// User functions should implement this trait
#[async_trait]
pub trait UserFunction {
    /// Call the userfunction, parameters are passed in as a Value
    async fn call(&self, params: Value) -> FunctionResult;

    /// The name of the user-function
    fn name(&self) -> &'static str;

    /// Indicates if results of this function can be cached,
    /// true by default
    fn cacheable(&self) -> bool {
        true
    }
}

pub type BoxedFunction = Box<dyn UserFunction + Send + Sync + 'static>;

/// Error type returned from UserFunction
#[derive(Debug, DisplayDoc, thiserror::Error)]
pub enum FunctionError {
    /// Invalid parameter {0:?}: Expected {1},
    InvalidParameter(Value, String),

    /// Unspecified er
    Unspecified(#[from] Box<dyn error::Error + Send + Sync>),
}

impl From<Error> for FunctionError {
    fn from(error: Error) -> Self {
        match error {
            Error::UnexpectedValueType(value, expected) => {
                FunctionError::InvalidParameter(value, expected)
            }
            err => FunctionError::Unspecified(err.into()),
        }
    }
}

/// Result type returned from UserFunction
pub type FunctionResult = result::Result<Value, FunctionError>;
