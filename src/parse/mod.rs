//! Parse rules and expressions written using the Reval DSL

mod expr;
mod helpers;
mod rule;
mod unescape;

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(reval);

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("Rule has no name")]
    MissingRuleName,

    #[error("Error parsing expression: {0}")]
    ExprParseError(String),

    #[error("Error parsing rule: {0}")]
    RuleParseError(String),

    #[error("Not a valid symbols file, expected a map definition")]
    SymbolsFileNotValid,
}
