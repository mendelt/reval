use crate::{
    error::{Error, Result},
    ruleset::RuleSet,
    value::Value,
};

pub(super) struct EvalContext<'a> {
    ruleset: &'a RuleSet,
    facts: &'a Value,
}

impl<'a> EvalContext<'a> {
    pub(super) fn new(ruleset: &'a RuleSet, facts: &'a Value) -> Self {
        Self { ruleset, facts }
    }
}

impl EvalContext<'_> {
    pub(super) async fn call_function(&self, name: &str, params: Value) -> Result<Value> {
        self.ruleset.call_function(name, params).await
    }

    pub(super) fn reference(&self, name: &str) -> Result<Value> {
        match self.facts {
            value if name == "facts" => Ok(value),
            Value::Map(facts) => facts
                .get(name)
                .ok_or_else(|| Error::UnknownRef(name.to_owned())),
            _ => Err(Error::InvalidType),
        }
        .cloned()
    }

    pub(super) fn symbol(&self, name: &str) -> Result<Value> {
        self.ruleset.get_symbol(name).cloned()
    }
}
