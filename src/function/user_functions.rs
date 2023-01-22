use super::BoxedFunction;
use crate::{Error, Result};
use std::collections::HashMap;

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
    pub fn add_function(&mut self, function: BoxedFunction) -> Result<()> {
        if self.functions.contains_key(function.name()) {
            return Err(Error::DuplicateFunctionName(function.name().to_string()));
        }

        // TODO: Check if function name is valid
        self.functions.insert(function.name(), function);

        Ok(())
    }

    pub fn add_functions<I: IntoIterator<Item = BoxedFunction>>(
        &mut self,
        functions: I,
    ) -> Result<()> {
        for function in functions {
            self.add_function(function)?;
        }

        Ok(())
    }

    /// Merge two sets of user-functions
    pub fn merge(&mut self, functions: UserFunctions) -> &mut Self {
        self.functions.extend(functions.functions.into_iter());
        self
    }
}
