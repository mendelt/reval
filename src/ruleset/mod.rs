//! Manage and evaluate a set of rules or expressions using RuleSets

mod builder;
mod rule;

pub use self::{
    builder::{ruleset, Builder},
    rule::Rule,
};
use crate::{
    error::Result,
    function::UserFunctions,
    symbol::Symbols,
    value::{ser::ValueSerializer, Value},
};
use serde::Serialize;

#[derive(Default)]
pub struct RuleSet {
    rules: Vec<Rule>,
    functions: UserFunctions,
    symbols: Symbols,
}

impl RuleSet {
    /// Evaluate the rules in the RuleSet against a piece of data
    pub async fn evaluate(&self, facts: &impl Serialize) -> Result<Vec<Outcome<'_>>> {
        self.evaluate_value(&facts.serialize(ValueSerializer)?)
            .await
    }

    pub async fn evaluate_value(&self, facts: &Value) -> Result<Vec<Outcome<'_>>> {
        let mut results = Vec::new();

        for rule in self.rules.iter() {
            results.push(Outcome {
                value: rule.expr().eval_rule(self, facts).await,
                rule,
            });
        }

        Ok(results)
    }

    pub(crate) async fn call_function(&self, name: &str, params: Value) -> Result<Value> {
        self.functions.call(name, params).await
    }

    pub(crate) fn get_symbol(&self, symbol: &str) -> Option<&Value> {
        self.symbols.get(symbol)
    }
}

/// The outcome from evaluating a rule.
/// Contains the resulting value from evaluating the rule expression plus
/// a reference to the rule that was evaluated to produce the data
pub struct Outcome<'a> {
    pub value: Result<Value>,
    pub rule: &'a Rule,
}
