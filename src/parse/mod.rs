//! Parse rules written using the Reval DSL

mod expr;
mod value;

use crate::expr::Expr;

impl Expr {
    /// Parse an expression written in the Reval DSL format
    pub fn parse(input: &str) -> Self {
        let (remaining, expr) = expr::expr(input).unwrap();
        assert!(remaining.is_empty());
        expr
    }
}
