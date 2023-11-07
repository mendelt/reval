//! Manage and evaluate a set of rules or expressions using RuleSets

pub mod builder;
pub mod rule;

use crate::{
    error::Result,
    expr::EvaluationContext,
    function::UserFunctions,
    symbol::Symbols,
    value::{ser::ValueSerializer, Value},
};
use serde::Serialize;

use self::rule::{Outcome, Rule};

pub struct RuleSet {
    rules: Vec<Rule>,
    functions: UserFunctions,
    symbols: Symbols,
}

impl RuleSet {
    /// Evaluate the rules in the RuleSet against a piece of data
    pub async fn evaluate(&self, facts: &impl Serialize) -> Result<Vec<Outcome>> {
        self.evaluate_value(&facts.serialize(ValueSerializer)?)
            .await
    }

    pub async fn evaluate_value(&self, facts: &Value) -> Result<Vec<Outcome>> {
        let mut context = EvaluationContext::init(&self.symbols, &self.functions);

        let mut results = Vec::new();

        for rule in self.rules.iter() {
            results.push(rule.evaluate(&mut context, facts).await);
        }

        Ok(results)
    }
}
