//! Parse rules and expressions written using the Reval DSL

mod expr;
mod rule;

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("Rule has no name")]
    MissingRuleName,

    #[error("Error parsing expression")]
    ExprParseError(String),
}
