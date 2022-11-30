//! Manage and evaluate a set of rules or expressions using RuleSets

pub mod builder;
pub mod rule;

use crate::{
    error::Result,
    function::{UserFunction, UserFunctions},
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
    /// Add a rule to the Ruleset
    pub fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule)
    }

    /// Add multiple rules to the RuleSet
    pub fn add_rules(&mut self, rules: impl IntoIterator<Item = Rule>) {
        for rule in rules {
            self.rules.push(rule)
        }
    }

    /// Add a user-function to the RuleSet
    pub fn add_function(&mut self, function: impl UserFunction + Send + Sync + 'static) {
        self.functions.add_function(function)
    }

    /// Add multiple user-functions to the RuleSet
    pub fn add_functions<I: IntoIterator<Item = F>, F: UserFunction + Send + Sync + 'static>(
        &mut self,
        functions: I,
    ) {
        self.functions.add_functions(functions);
    }

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
