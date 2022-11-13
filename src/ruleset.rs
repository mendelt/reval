use crate::{
    expr::{Error, Expr},
    value::Value,
};

/// A rule is an expression with a name
pub struct Rule {
    _name: String,
    expr: Expr,
}

/// A set of expressions
pub struct RuleSet {
    rules: Vec<Rule>,
}

impl Default for RuleSet {
    fn default() -> Self {
        Self { rules: Vec::new() }
    }
}

impl RuleSet {
    pub fn evaluate(&self, facts: &Value) -> Result<Vec<Value>, Error> {
        self.rules
            .iter()
            .map(|rule| rule.expr.evaluate(facts))
            .collect()
    }
}
