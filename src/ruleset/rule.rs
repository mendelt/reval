use crate::expr::Expr;

/// A rule is an expression with a name
#[derive(Debug, Clone, PartialEq)]
pub struct Rule {
    pub(super) name: String,
    description: Option<String>,
    expr: Expr,
}

impl Rule {
    /// Construct a new rule from a name and an expression
    pub fn new(
        name: impl Into<String>,
        description: impl Into<Option<String>>,
        expr: Expr,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            expr,
        }
    }

    /// Return the name of the rule
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return an optional description of the rule
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn expr(&self) -> &Expr {
        &self.expr
    }
}
