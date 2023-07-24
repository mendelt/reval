//! Parse rules and expressions written using the Reval DSL

mod expr;
mod rule;

/// Extension crate for Reval DSL parsing, adds a `parse` method to Expr and Rule
pub trait Parsers {
    type Error;
    type Parseable;

    fn parse(input: &str) -> Result<Self::Parseable, Self::Error>;
}
