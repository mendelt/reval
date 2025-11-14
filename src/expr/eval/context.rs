use crate::{
    error::{Error, Result},
    ruleset::RuleSet,
    symbol::Symbols,
    value::Value,
};

#[derive(Clone)]
pub(super) struct EvalContext<'a> {
    /// The ruleset that we're evaluating against
    ruleset: &'a RuleSet,

    /// The tree of values we're evaluating the rules on
    facts: &'a Value,

    /// Scope adds symbols that are only valid at the current level of evaluation
    scope: Symbols,
}

impl<'a> EvalContext<'a> {
    pub(super) fn new(ruleset: &'a RuleSet, facts: &'a Value) -> Self {
        Self {
            ruleset,
            facts,
            scope: Symbols::default(),
        }
    }
}

impl EvalContext<'_> {
    pub(super) async fn call_function(&self, name: &str, params: Value) -> Result<Value> {
        self.ruleset.call_function(name, params).await
    }

    pub(super) fn get_reference(&self, name: &str) -> Result<Value> {
        self.scope
            .get(name)
            .cloned()
            .ok_or_else(|| Error::UnknownRef(name.to_owned()))
            .or_else(|_| {
                match self.facts {
                    value if name == "facts" => Ok(value),
                    Value::Map(facts) => facts
                        .get(name)
                        .ok_or_else(|| Error::UnknownRef(name.to_owned())),
                    _ => Err(Error::InvalidType),
                }
                .cloned()
            })
    }

    pub(super) fn get_symbol(&self, name: &str) -> Result<Value> {
        self.ruleset
            .get_symbol(name)
            .cloned()
            .ok_or_else(|| Error::UnknownSymbol(name.to_owned()))
    }

    pub(super) fn start_scope(
        &self,
        values: impl IntoIterator<Item = (impl ToString, Value)>,
    ) -> EvalContext<'_> {
        let mut new_scope = self.scope.clone();
        new_scope.append(values);

        EvalContext {
            ruleset: self.ruleset,
            facts: self.facts,
            scope: new_scope,
        }
    }
}
