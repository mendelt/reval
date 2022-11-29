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

    /// The name of the user-function
    fn name(&self) -> &'static str;
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

/// Stores user-functions so they can be easilly called
#[derive(Default)]
pub struct UserFunctions {
    functions: HashMap<&'static str, Box<dyn UserFunction + Send + Sync>>,
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

    /// Add a user-function to the collection
    pub fn add_function<F: UserFunction + Send + Sync + 'static>(&mut self, function: F) {
        // TODO: Check if function name is valid
        self.functions.insert(function.name(), Box::new(function));
    }

    pub fn add_functions<I: IntoIterator<Item = F>, F: UserFunction + Send + Sync + 'static>(
        &mut self,
        functions: I,
    ) {
        for function in functions {
            self.add_function(function);
        }
    }

    /// Merge two sets of user-functions
    pub fn merge(&mut self, functions: UserFunctions) -> &mut Self {
        self.functions.extend(functions.functions.into_iter());
        self
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
