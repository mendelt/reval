use crate::{expr::Expr, value::Value};
use std::collections::BTreeMap;

/// A rule is an expression with a name
#[derive(Debug, Clone, PartialEq)]
pub struct Rule {
    pub(super) name: String,
    metadata: BTreeMap<String, Value>,
    expr: Expr,
}

impl Rule {
    /// Construct a new rule from a name and an expression
    pub fn new(name: impl Into<String>, metadata: BTreeMap<String, Value>, expr: Expr) -> Self {
        Self {
            name: name.into(),
            metadata,
            expr,
        }
    }

    /// Return the name of the rule
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Read a metadata field by name
    pub fn get_metadata(&self, field: &str) -> Option<&Value> {
        self.metadata.get(field)
    }

    /// Return an optional description of the rule
    pub fn description(&self) -> Option<&str> {
        match self.metadata.get("description") {
            Some(Value::String(description)) => Some(description.as_str()),
            _ => None,
        }
    }

    pub fn expr(&self) -> &Expr {
        &self.expr
    }
}
