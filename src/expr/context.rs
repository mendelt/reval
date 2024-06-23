use crate::{
    expr::Expr,
    function::{FunctionCache, UserFunctions, EMPTY_FUNCTIONS},
    symbol::{Symbols, EMPTY_SYMBOLS},
    value::Value,
    Result,
};

/// Function state during rule invocations
pub(crate) struct EvaluationContext<'a> {
    symbols: &'a Symbols,
    functions: &'a UserFunctions,
}

impl<'a> EvaluationContext<'a> {
    pub(crate) fn init(symbols: &'a Symbols, functions: &'a UserFunctions) -> Self {
        Self { symbols, functions }
    }

    pub(crate) async fn call_function(
        &self,
        name: &str,
        params: Value,
        function_cache: &mut FunctionCache,
    ) -> Result<Value> {
        self.functions.call(name, params, function_cache).await
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
        }
    }
}
