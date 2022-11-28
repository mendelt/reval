//! Manage and evaluate a set of rules or expressions using RuleSets

pub mod builder;
pub mod rule;

use crate::{
    error::Result,
    function::{UserFunction, UserFunctions},
    value::{ser::ValueSerializer, Value},
};
use serde::Serialize;

use self::rule::Rule;

/// The RuleSet type can
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
        let mut context = (&self.functions).into();

        let mut results = Vec::new();

        for rule in self.rules.iter() {
            results.push(rule.expr.evaluate(&mut context, facts).await?);
        }

        Ok(results)
    }
}
