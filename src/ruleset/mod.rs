//! Manage and evaluate a set of rules or expressions using RuleSets

pub mod builder;
pub mod rule;

use crate::{
    error::Result,
    function::UserFunctions,
    value::{ser::ValueSerializer, Value},
};
use serde::Serialize;

use self::rule::{Outcome, Rule};

/// The RuleSet type can
pub struct RuleSet {
    rules: Vec<Rule>,
    functions: UserFunctions,
}

impl RuleSet {
    /// Evaluate the rules in the RuleSet against a piece of data
    pub async fn evaluate(&self, facts: &impl Serialize) -> Result<Vec<Outcome>> {
        self.evaluate_value(&facts.serialize(ValueSerializer)?)
            .await
    }

    pub async fn evaluate_value(&self, facts: &Value) -> Result<Vec<Outcome>> {
        let mut context = (&self.functions).into();

        let mut results = Vec::new();

        for rule in self.rules.iter() {
            results.push(rule.evaluate(&mut context, facts).await);
        }

        Ok(results)
    }
}
