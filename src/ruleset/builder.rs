use crate::{
    error::{Error, Result},
    expr::Expr,
    function::{UserFunction, UserFunctions},
    ruleset::{rule::Rule, RuleSet},
    symbol::Symbols,
};

/// Start building a ruleset
pub fn ruleset() -> Builder {
    Builder {
        rules: Vec::new(),
        functions: Default::default(),
        symbols: Default::default(),
    }
}

/// Ruleset builder
pub struct Builder {
    rules: Vec<Rule>,
    functions: UserFunctions,
    symbols: Symbols,
}

impl Builder {
    /// Add a rule to the ruleset
    pub fn with_rule(mut self, rule: Rule) -> Result<Self> {
        let name = rule.name();

        if self.rules.iter().any(|r| r.name() == name) {
            return Err(Error::DuplicateRuleName(rule.name));
        }

        self.rules.push(rule);
        Ok(self)
    }

    /// Add multiple rules to the RuleSet
    pub fn with_rules(mut self, rules: impl IntoIterator<Item = Rule>) -> Result<Self> {
        for rule in rules {
            self = self.with_rule(rule)?;
        }
        Ok(self)
    }

    /// Add a user-function to the ruleset
    pub fn with_function(
        mut self,
        function: impl UserFunction + Send + Sync + 'static,
    ) -> Result<Self> {
        self.functions.add_function(function)?;
        Ok(self)
    }

    /// Add multiple boxed user-functions to the ruleset
    pub fn with_functions(
        mut self,
        functions: impl IntoIterator<Item = Box<dyn UserFunction + Send + Sync + 'static>>,
    ) -> Result<Self> {
        for function in functions {
            self.functions.add_boxed_function(function)?;
        }
        Ok(self)
    }

    pub fn with_symbol(mut self, symbol: impl ToString, value: Expr) -> Self {
        self.symbols.insert(symbol, value);
        self
    }

    pub fn with_symbols(mut self, values: impl IntoIterator<Item = (impl ToString, Expr)>) -> Self {
        self.symbols.append(values);
        self
    }

    /// Finalize the builder and create the RuleSet
    pub fn build(self) -> RuleSet {
        RuleSet {
            rules: self.rules,
            functions: self.functions,
            symbols: self.symbols,
        }
    }
}

#[cfg(test)]
pub mod when_building_ruleset {
    use super::*;
    use crate::value::Value;

    /// Test helper that creates an empty rule
    fn rule(name: &str) -> Rule {
        Rule::new(name, None, Value::None.into())
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
