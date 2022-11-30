//! User functions

use crate::{
    error::{Error, Result},
    value::Value,
};
use async_trait::async_trait;
use displaydoc::Display as DisplayDoc;
use std::error;
use std::{collections::HashMap, result};

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

/// Stores user-functions so they can be easilly called
#[derive(Default)]
pub struct UserFunctions {
    functions: HashMap<&'static str, BoxedFunction>,
}

impl UserFunctions {
    /// Get a userfunction by name
    pub fn get(&self, name: &str) -> Result<&BoxedFunction> {
        self.functions
            .get(name)
            .ok_or_else(|| Error::UnknownUserFunction(name.to_owned()))
    }

    /// Add a user-function to the collection
    pub fn add_function(&mut self, function: BoxedFunction) {
        // TODO: Check if function name is valid
        self.functions.insert(function.name(), function);
    }

    pub fn add_functions<I: IntoIterator<Item = BoxedFunction>>(&mut self, functions: I) {
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
        Self {
            functions,
            results: HashMap::new(),
        }
    }
}

/// Function state during rule invocations
pub struct FunctionContext<'a> {
    functions: &'a UserFunctions,

    /// Store results of previously evaluated user-functions here by
    /// function-name and parameter-value
    results: HashMap<String, Value>,
}

async fn call_function(function: &BoxedFunction, params: Value, name: &str) -> Result<Value> {
    function
        .call(params)
        .await
        .map_err(|err| Error::UserFunctionError(name.to_owned(), err))
}

impl<'a> FunctionContext<'a> {
    pub(crate) async fn call(&mut self, name: &str, params: Value) -> Result<Value> {
        let function = self.functions.get(name)?;

        if function.cacheable() {
            let cache_key = format!("{name}-{params:?}");

            match self.results.get(&cache_key) {
                Some(value) => Ok(value.clone()),
                None => {
                    let result = call_function(function, params, name).await?;
                    self.results.insert(cache_key, result.clone());
                    Ok(result)
                }
            }
        } else {
            call_function(function, params, name).await
        }
    }
}
