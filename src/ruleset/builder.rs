use crate::{
    error::{Error, Result},
    function::{UserFunction, UserFunctions},
    ruleset::{rule::Rule, RuleSet},
};

/// Start building a ruleset
pub fn ruleset() -> Builder {
    Builder {
        rules: Vec::new(),
        functions: UserFunctions::default(),
    }
}

/// Ruleset builder
pub struct Builder {
    rules: Vec<Rule>,
    functions: UserFunctions,
}

impl Builder {
    /// Add a rule to the ruleset being built
    pub fn with_rule(mut self, rule: Rule) -> Result<Self> {
        if self.rules.iter().any(|r| r.name() == rule.name()) {
            return Err(Error::DuplicateRuleName(rule.name));
        }

        self.rules.push(rule);
        Ok(self)
    }

    /// Add multiple rules to the RuleSet being built
    pub fn with_rules(mut self, rules: impl IntoIterator<Item = Rule>) -> Result<Self> {
        for rule in rules {
            self = self.with_rule(rule)?;
        }
        Ok(self)
    }

    /// Add a user-function to the ruleset being built
    pub fn with_function(
        mut self,
        function: impl UserFunction + Send + Sync + 'static,
    ) -> Result<Self> {
        self.functions.add_function(function)?;
        Ok(self)
    }

    /// Finalize the builder and create the RuleSet
    pub fn build(self) -> RuleSet {
        RuleSet {
            rules: self.rules,
            functions: self.functions,
        }
    }
}
