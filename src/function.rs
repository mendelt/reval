//! User functions

use crate::{
    error::{Error, Result},
    value::Value,
};
use async_trait::async_trait;
use displaydoc::Display as DisplayDoc;
use std::{collections::HashMap, result};

/// User functions should implement this trait
#[async_trait]
pub trait UserFunction {
    /// Call the userfunction, parameters are passed in as a Value
    async fn call(&self, params: Value) -> FunctionResult;
}

/// Error type returned from UserFunction
#[derive(Debug, DisplayDoc, thiserror::Error)]
pub enum FunctionError {
    /// Invalid parameter {0:?}: Expected {1},
    InvalidParameter(Value, String),

    /// Unspecified er
    Unspecified(#[from] anyhow::Error),
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

/// Stores user-functions by name
#[derive(Default)]
pub struct UserFunctions {
    functions: HashMap<String, Box<dyn UserFunction + Send + Sync>>,
}

impl UserFunctions {
    /// Call a user-function by name.
    pub async fn call(&self, name: &str, params: Value) -> Result<Value> {
        match self.functions.get(name) {
            Some(fun) => fun
                .call(params)
                .await
                .map_err(|err| Error::UserFunctionError(name.to_owned(), err)),
            None => Err(Error::UnknownUserFunction(name.to_owned())),
        }
    }

    pub fn add<F: UserFunction + Send + Sync + 'static>(&mut self, name: &str, function: F) {
        self.functions.insert(name.to_owned(), Box::new(function));
    }
}

impl<'a> From<&'a UserFunctions> for FunctionContext<'a> {
    fn from(functions: &'a UserFunctions) -> Self {
        Self { functions }
    }
}

/// Function state during rule invocations
pub struct FunctionContext<'a> {
    functions: &'a UserFunctions,
}

impl<'a> FunctionContext<'a> {
    pub(crate) async fn call(&mut self, function: &str, params: Value) -> Result<Value> {
        self.functions.call(function, params).await
    }
}
