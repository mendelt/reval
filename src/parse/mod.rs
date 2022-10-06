mod binary;
mod value;

use crate::expr::Expr;
use nom::{branch::alt, IResult};

pub fn parse(input: &str) -> Expr {
    let (remaining, expr) = expr(input).unwrap();
    assert!(remaining.is_empty());
    expr
}

fn expr(input: &str) -> IResult<&str, Expr> {
    alt((binary::binary, value::value))(input)
}

