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
        map(
            separated_pair(add_expr, char('*'), mul_expr),
            |(left, right)| Expr::Binary(BinaryOperator::Mult, Box::new(left), Box::new(right)),
        ),
        add_expr,
    ))(input)
}

fn add_expr(input: &str) -> IResult<&str, Expr> {
    alt((
        map(
            separated_pair(value_expr, char('+'), add_expr),
            |(left, right)| Expr::Binary(BinaryOperator::Add, Box::new(left), Box::new(right)),
        ),
        value_expr,
    ))(input)
}

fn value_expr(input: &str) -> IResult<&str, Expr> {
    map(value, |value| Expr::Value(value))(input)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::value::{Number, Value};

    #[test]
    fn should_parse_multiplication() {
        let (rest, parsed) = expr("1*4").unwrap();

        assert!(rest.is_empty());
        assert_eq!(
            parsed,
            Expr::Binary(
                BinaryOperator::Mult,
                Box::new(Expr::Value(Value::Number(Number::Int(1)))),
                Box::new(Expr::Value(Value::Number(Number::Int(4)))),
            )
        );
    }

    #[test]
    fn should_parse_addition() {
        let (rest, parsed) = expr("1+4").unwrap();

        assert!(rest.is_empty());
        assert_eq!(
            parsed,
            Expr::Binary(
                BinaryOperator::Add,
                Box::new(Expr::Value(Value::Number(Number::Int(1)))),
                Box::new(Expr::Value(Value::Number(Number::Int(4)))),
            ),
        );
    }
}
