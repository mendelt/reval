//! User functions
use crate::{
    expr::keywords::{is_reserved_keyword, is_valid_identifier},
    value::Value,
    Error, Result,
};
use async_trait::async_trait;
use std::collections::BTreeMap;
use std::result;

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

/// Result type returned from UserFunction
pub type FunctionResult = result::Result<Value, anyhow::Error>;

pub(crate) type FunctionCache = BTreeMap<String, Value>;

/// Stores user-functions so they can be easilly called
#[derive(Default)]
pub(crate) struct UserFunctions {
    functions: BTreeMap<&'static str, BoxedFunction>,
}

impl UserFunctions {
    /// Get a userfunction by name
    pub(crate) fn get(&self, name: &str) -> Result<&BoxedFunction> {
        self.functions
            .get(name)
            .ok_or_else(|| Error::UnknownUserFunction(name.to_owned()))
    }

    /// Add a user-function to the collection
    pub(crate) fn add_function(
        &mut self,
        function: impl UserFunction + Send + Sync + 'static,
    ) -> Result<()> {
        self.add_boxed_function(Box::new(function))
    }

    pub(crate) fn add_boxed_function(&mut self, function: BoxedFunction) -> Result<()> {
        let name = function.name();

        if is_reserved_keyword(name) {
            return Err(Error::InvalidFunctionName(name.to_string()));
        }

        if !is_valid_identifier(name) {
            return Err(Error::InvalidFunctionName(name.to_string()));
        }

        if self.functions.contains_key(name) {
            return Err(Error::DuplicateFunctionName(name.to_string()));
        }

        self.functions.insert(name, function);

        Ok(())
    }

    pub(crate) async fn call(
        &self,
        name: &str,
        param: Value,
        results_cache: &mut FunctionCache,
    ) -> Result<Value> {
        let function = self.get(name)?;

        if function.cacheable() {
            let cache_key = format!("{name}-{param:?}");

            match results_cache.get(&cache_key) {
                Some(value) => Ok(value.clone()),
                None => {
                    let result = call_function(function, param, name).await?;
                    results_cache.insert(cache_key, result.clone());
                    Ok(result)
                }
            }
        } else {
            call_function(function, param, name).await
        }
    }
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

/// Convenience type for passing around boxed user-function implementations
pub(crate) type BoxedFunction = Box<dyn UserFunction + Send + Sync + 'static>;

#[cfg(test)]
mod when_managing_user_functions {
    use super::*;
    use async_trait::async_trait;

    #[test]
    fn should_add_function() {
        let mut functions = UserFunctions::default();

        functions.add_function(func("test_function")).unwrap();
        functions.get("test_function").unwrap();
    }

    #[test]
    fn should_not_add_duplicate_function_name() {
        let mut functions = UserFunctions::default();

        // Add a function
        functions.add_function(func("test_function")).unwrap();

        // Add a function with the same name
        assert!(matches!(
            functions.add_function(func("test_function")),
            Err(Error::DuplicateFunctionName(name)) if name == "test_function".to_string()
        ));
    }

    #[test]
    fn should_not_add_reserved_keyword_as_function_name() {
        let mut functions = UserFunctions::default();

        // Add a function with the same name
        assert!(matches!(
            functions.add_function(func("if")),
            Err(Error::InvalidFunctionName(name)) if name == "if".to_string()
        ));
    }

    fn func(name: &'static str) -> TestFunc {
        TestFunc { name }
    }

    struct TestFunc {
        name: &'static str,
    }

    #[async_trait]
    impl UserFunction for TestFunc {
        async fn call(&self, _params: Value) -> FunctionResult {
            Ok(Value::None)
        }

        fn name(&self) -> &'static str {
            self.name
        }
    }
}
