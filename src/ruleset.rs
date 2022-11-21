use crate::{
    expr::Expr,
    function::{UserFunction, UserFunctions},
    Result, Value,
};

/// A rule is an expression with a name
#[derive(Debug, Clone, PartialEq)]
pub struct Rule {
    name: String,
    expr: Expr,
}

impl Rule {
    pub fn new(name: impl Into<String>, expr: Expr) -> Self {
        Self {
            name: name.into(),
            expr,
        }
    }
}

/// A set of expressions
#[derive(Default)]
pub struct RuleSet {
    rules: Vec<Rule>,
    functions: UserFunctions,
}

impl RuleSet {
    pub fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule)
    }

    pub fn add_function(
        &mut self,
        name: &str,
        function: impl UserFunction + Send + Sync + 'static,
    ) {
        self.functions.add(name, function)
    }

    pub async fn evaluate(&self, facts: &Value) -> Result<Vec<Value>> {
        let mut context = EvalContext {
            functions: &self.functions,
        };

        let mut results = Vec::new();

        for rule in self.rules.iter() {
            results.push(rule.expr.evaluate(&mut context, facts).await?);
        }

        Ok(results)
    }
}

/// Evaluation context
pub struct EvalContext<'a> {
    functions: &'a UserFunctions,
}

impl<'a> EvalContext<'a> {
    pub async fn call(&mut self, function: &str, params: Value) -> Result<Value> {
        self.functions.call(function, params).await
    }
}
