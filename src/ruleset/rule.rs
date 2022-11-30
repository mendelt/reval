use crate::{expr::Expr, function::FunctionContext, value::Value, Result};

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

    pub async fn evaluate<'a>(&self, context: &mut FunctionContext<'a>, facts: &Value) -> Outcome {
        Outcome {
            value: self.expr.evaluate(context, facts).await,
            rule: &self.name,
        }
    }
}

/// The outcome from evaluating a rule.
/// Contains the resulting value from evaluating the rule expression plus
/// metadata. For now the metadata is limited to the name of the rule
pub struct Outcome<'a> {
    pub value: Result<Value>,
    pub rule: &'a str,
}
