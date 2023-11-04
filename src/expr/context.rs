use crate::{
    function::{UserFunctions, EMPTY_FUNCTIONS},
    value::Value,
    Result,
};
use std::collections::HashMap;

/// Function state during rule invocations
pub struct EvaluationContext<'a> {
    functions: &'a UserFunctions,

    /// Store results of previously evaluated user-functions here by
    /// function-name and parameter-value
    function_cache: HashMap<String, Value>,
}

impl<'a> EvaluationContext<'a> {
    pub(crate) async fn call_function(&mut self, name: &str, params: Value) -> Result<Value> {
        self.functions
            .call(name, params, &mut self.function_cache)
            .await
    }
}

impl Default for EvaluationContext<'_> {
    fn default() -> Self {
        Self {
            functions: &EMPTY_FUNCTIONS,
            function_cache: Default::default(),
        }
    }
}

impl<'a> From<&'a UserFunctions> for EvaluationContext<'a> {
    fn from(functions: &'a UserFunctions) -> Self {
        Self {
            functions,
            function_cache: HashMap::new(),
        }
    }
}
