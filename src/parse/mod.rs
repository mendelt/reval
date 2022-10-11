mod expr;
mod value;

use crate::expr::Expr;

pub fn parse(input: &str) -> Expr {
    let (remaining, expr) = expr::expr(input).unwrap();
    assert!(remaining.is_empty());
    expr
}
