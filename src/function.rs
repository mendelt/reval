//! User functions

use crate::{
    error::{Error, Result},
    value::Value,
};
use async_trait::async_trait;
use std::{collections::HashMap, result};

/// User functions should implement this type
#[async_trait]
pub trait UserFunction {
    /// Call the userfunction, parameters are passed in as a Value
    async fn call(&self, params: Value) -> result::Result<Value, anyhow::Error>;
}

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
pub(crate) struct FunctionContext<'a> {
    functions: &'a UserFunctions,
}

impl<'a> FunctionContext<'a> {
    pub(crate) async fn call(&mut self, function: &str, params: Value) -> Result<Value> {
        self.functions.call(function, params).await
    }
}
