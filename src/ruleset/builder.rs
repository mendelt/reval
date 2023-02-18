use crate::{
    error::{Error, Result},
    expr::keywords::{is_reserved_keyword, is_valid_identifier},
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
        let name = rule.name();

        if is_reserved_keyword(name) {
            return Err(Error::InvalidRuleName(name.to_string()));
        }

        if is_valid_identifier(name) {
            return Err(Error::InvalidRuleName(name.to_string()));
        }

        if self.rules.iter().any(|r| r.name() == name) {
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

#[cfg(test)]
pub mod when_building_ruleset {
    use super::*;
    use crate::prelude::*;

    /// Test helper that creates an empty rule
    fn rule(name: &str) -> Rule {
        Rule::new(name, Value::None.into())
    }

    #[test]
    fn should_add_rule() {
        ruleset().with_rule(rule("test rule 1")).unwrap();
    }

    #[test]
    fn should_add_multiple_rules() {
        let builder = ruleset().with_rule(rule("test rule 1")).unwrap();

        builder.with_rule(rule("test rule 2")).unwrap();
    }

    #[test]
    fn should_not_add_duplicate_rule_name() {
        // Start a ruleset builder and add one rule
        let builder = ruleset().with_rule(rule("test rule 1")).unwrap();

        assert!(matches!(
            builder.with_rule(rule("test rule 1")),
            Err(Error::DuplicateRuleName(name)) if name == "test rule 1".to_string()
        ));
    }
}
