use crate::{
    expr::{Error, Expr},
    value::Value,
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

    pub fn evaluate(&self, facts: &Value) -> Result<Value, Error> {
        self.expr.evaluate(facts)
    }
}

/// A set of expressions
#[derive(Debug, Default, Clone, PartialEq)]
pub struct RuleSet {
    rules: Vec<Rule>,
}

impl RuleSet {
    pub fn evaluate(&self, facts: &Value) -> Result<Vec<Value>, Error> {
        self.rules
            .iter()
            .map(|rule| rule.expr.evaluate(facts))
            .collect()
    }
}
