use crate::{
    expr::Expr,
    function::{UserFunctions, EMPTY_FUNCTIONS},
    symbol::{Symbols, EMPTY_SYMBOLS},
    value::Value,
    Result,
};
use std::collections::HashMap;

/// Function state during rule invocations
pub struct EvaluationContext<'a> {
    symbols: &'a Symbols,
    functions: &'a UserFunctions,

    /// Store results of previously evaluated user-functions here by
    /// function-name and parameter-value
    function_cache: HashMap<String, Value>,
}

impl<'a> EvaluationContext<'a> {
    pub(crate) fn init(symbols: &'a Symbols, functions: &'a UserFunctions) -> Self {
        Self {
            symbols,
            functions,
            function_cache: Default::default(),
        }
    }

    pub(crate) async fn call_function(&mut self, name: &str, params: Value) -> Result<Value> {
        self.functions
            .call(name, params, &mut self.function_cache)
            .await
    }

    pub(crate) fn get_symbol(&self, symbol: &str) -> Result<&Expr> {
        self.symbols.get(symbol)
    }
}

impl Default for EvaluationContext<'_> {
    fn default() -> Self {
        Self {
            symbols: &EMPTY_SYMBOLS,
            functions: &EMPTY_FUNCTIONS,
            function_cache: Default::default(),
        }
    }
}
