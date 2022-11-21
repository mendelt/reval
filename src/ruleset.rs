use serde::Serialize;

use crate::{
    error::Result,
    expr::Expr,
    function::{UserFunction, UserFunctions},
    value::{ser::ValueSerializer, Value},
};

/// A set of expressions
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

    pub async fn evaluate(&self, facts: &impl Serialize) -> Result<Vec<Value>> {
        self.evaluate_value(&facts.serialize(ValueSerializer)?)
            .await
    }

    pub async fn evaluate_value(&self, facts: &Value) -> Result<Vec<Value>> {
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

pub fn ruleset() -> Builder {
    Builder {
        rules: Vec::new(),
        functions: UserFunctions::default(),
    }
}

pub struct Builder {
    rules: Vec<Rule>,
    functions: UserFunctions,
}

impl Builder {
    pub fn with_rule(mut self, rule: Rule) -> Builder {
        self.rules.push(rule);
        self
    }

    pub fn with_function(
        &mut self,
        name: &str,
        function: impl UserFunction + Send + Sync + 'static,
    ) {
        self.functions.add(name, function)
    }

    pub fn build(self) -> RuleSet {
        RuleSet {
            rules: self.rules,
            functions: self.functions,
        }
    }
}

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

/// Evaluation context
pub struct EvalContext<'a> {
    functions: &'a UserFunctions,
}

impl<'a> EvalContext<'a> {
    pub(crate) async fn call(&mut self, function: &str, params: Value) -> Result<Value> {
        self.functions.call(function, params).await
    }
}
