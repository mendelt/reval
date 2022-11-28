use crate::{
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
    pub fn with_rule(mut self, rule: Rule) -> Builder {
        self.rules.push(rule);
        self
    }

    /// Add a user-function to the ruleset being built
    pub fn with_function(
        mut self,
        name: &str,
        function: impl UserFunction + Send + Sync + 'static,
    ) -> Builder {
        self.functions.add(name, function);
        self
    }

    pub fn build(self) -> RuleSet {
        RuleSet {
            rules: self.rules,
            functions: self.functions,
        }
    }
}
