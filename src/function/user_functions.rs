use crate::{Error, Result};
use std::collections::HashMap;

use super::UserFunction;

/// Stores user-functions so they can be easilly called
#[derive(Default)]
pub struct UserFunctions {
    functions: HashMap<&'static str, BoxedFunction>,
}

impl UserFunctions {
    /// Get a userfunction by name
    pub(crate) fn get(&self, name: &str) -> Result<&BoxedFunction> {
        self.functions
            .get(name)
            .ok_or_else(|| Error::UnknownUserFunction(name.to_owned()))
    }

    /// Add a user-function to the collection
    pub fn add_function(
        &mut self,
        function: impl UserFunction + Send + Sync + 'static,
    ) -> Result<()> {
        if self.functions.contains_key(function.name()) {
            return Err(Error::DuplicateFunctionName(function.name().to_string()));
        }

        // TODO: Check if function name is valid
        self.functions.insert(function.name(), Box::new(function));

        Ok(())
    }

    /// Merge two sets of user-functions
    pub fn merge(&mut self, functions: UserFunctions) -> &mut Self {
        self.functions.extend(functions.functions.into_iter());
        self
    }
}

/// Convenience type for passing around boxed user-function implementations
pub(crate) type BoxedFunction = Box<dyn UserFunction + Send + Sync + 'static>;

#[cfg(test)]
mod when_managing_user_functions {
    use super::*;
    use crate::prelude::*;
    use async_trait::async_trait;

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

    #[test]
    fn should_add_function() {
        let mut functions = UserFunctions::default();

        assert!(functions
            .add_function(TestFunc {
                name: "test function",
            })
            .is_ok());
        assert!(functions.get("test function").is_ok());
    }

    #[test]
    fn should_not_add_duplicate_function_name() {
        let mut functions = UserFunctions::default();

        // Add a function
        assert!(functions
            .add_function(TestFunc {
                name: "test function",
            })
            .is_ok());

        // Add a function with the same name
        assert!(matches!(
            functions.add_function(TestFunc {
                name: "test function"
            }),
            Err(Error::DuplicateFunctionName(name)) if name == "test function".to_string()
        ));
    }
}
