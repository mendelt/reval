use super::{BoxedFunction, UserFunctions};
use crate::{value::Value, Error, Result};
use std::collections::HashMap;

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
        .map_err(|err| Error::UserFunctionError {
            function: name.to_owned(),
            error: err,
        })
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

impl<'a> From<&'a UserFunctions> for FunctionContext<'a> {
    fn from(functions: &'a UserFunctions) -> Self {
        Self {
            functions,
            results: HashMap::new(),
        }
    }
}
