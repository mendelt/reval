//! Parse rules and expressions written using the Reval DSL

mod expr;
mod rule;

#[derive(Debug, PartialEq, Eq, displaydoc::Display, thiserror::Error)]
pub enum Error {
    /// Rule has no name specified
    MissingRuleName,

    /// Error parsing expression
    ExprParseError(String),
}
