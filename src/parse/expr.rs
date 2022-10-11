use crate::expr::BinaryOperator;
use crate::expr::Expr;
use crate::parse::value::value;
use nom::{
    branch::alt, character::complete::char, combinator::map, sequence::separated_pair, IResult,
};

pub fn expr(input: &str) -> IResult<&str, Expr> {
    mul_expr(input)
}

fn mul_expr(input: &str) -> IResult<&str, Expr> {
    alt((
        add_expr,
        map(
            separated_pair(add_expr, char('*'), mul_expr),
            |(left, right)| Expr::Binary(BinaryOperator::Add, Box::new(left), Box::new(right)),
        ),
    ))(input)
}

fn add_expr(input: &str) -> IResult<&str, Expr> {
    alt((
        map(
            separated_pair(value_expr, char('+'), add_expr),
            |(left, right)| Expr::Binary(BinaryOperator::Add, Box::new(left), Box::new(right)),
        ),
        value_expr
    ))(input)
}

fn value_expr(input: &str) -> IResult<&str, Expr> {
    map(value, |value| Expr::Value(value))(input)
}

#[cfg(test)]
mod test {
    use crate::{
        parse::parse,
        value::{Number, Value},
    };

    use super::*;

    #[test]
    fn should_parse_addition() {
        assert_eq!(
            parse("1+4"),
            Expr::Binary(
                BinaryOperator::Add,
                Box::new(Expr::Value(Value::Number(Number::Int(1)))),
                Box::new(Expr::Value(Value::Number(Number::Int(4)))),
            ),
        );
    }
}
