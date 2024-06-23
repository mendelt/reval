//! Manage and evaluate a set of rules or expressions using RuleSets

mod builder;
mod rule;

pub use self::{
    builder::{ruleset, Builder},
    rule::Rule,
};
use crate::{
    error::Result,
    expr::EvaluationContext,
    function::UserFunctions,
    symbol::Symbols,
    value::{ser::ValueSerializer, Value},
};
use serde::Serialize;

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
            results.push(Outcome {
                value: rule.expr().evaluate(&mut context, facts).await,
                rule: &rule.name,
            });
        }

        Ok(results)
    }
}

/// The outcome from evaluating a rule.
/// Contains the resulting value from evaluating the rule expression plus
/// metadata. For now the metadata is limited to the name of the rule
pub struct Outcome<'a> {
    pub value: Result<Value>,
    pub rule: &'a str,
}
