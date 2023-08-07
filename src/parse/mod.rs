//! Parse rules and expressions written using the Reval DSL

mod expr;
mod rule;

/// Extension trait for Reval DSL parsing, adds a `parse` method to Expr and Rule
pub trait Parsers {
    type Parseable;

    fn parse(input: &str) -> Result<Self::Parseable, Error>;
}

#[derive(Debug, PartialEq, Eq, displaydoc::Display, thiserror::Error)]
pub enum Error {
    /// Rule has no name specified
    MissingRuleName,

    /// Error parsing expression
    ExprParseError(String),
}
