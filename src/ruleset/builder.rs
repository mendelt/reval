use crate::{
    function::{BoxedFunction, UserFunctions},
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
    pub fn with_rule(mut self, rule: Rule) -> Self {
        self.rules.push(rule);
        self
    }

    /// Add multiple rules to the RuleSet being built
    pub fn with_rules(mut self, rules: impl IntoIterator<Item = Rule>) -> Self {
        for rule in rules {
            self.rules.push(rule)
        }
        self
    }

    /// Add a user-function to the ruleset being built
    pub fn with_function(mut self, function: BoxedFunction) -> Builder {
        self.functions.add_function(function);
        self
    }

    /// Add multiple user-functions to the ruleset being built
    pub fn with_functions<I: IntoIterator<Item = BoxedFunction>>(mut self, functions: I) -> Self {
        self.functions.add_functions(functions);
        self
    }

    /// Finalize the builder and create the RuleSet
    pub fn build(self) -> RuleSet {
        RuleSet {
            rules: self.rules,
            functions: self.functions,
        }
    }
}
