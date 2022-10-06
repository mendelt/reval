use super::expr;
use crate::expr::BinaryOperator;
use crate::expr::Expr;
use nom::{character::complete::char, combinator::map, sequence::separated_pair, IResult};

pub fn binary(input: &str) -> IResult<&str, Expr> {
    add(input)
}

fn add(input: &str) -> IResult<&str, Expr> {
    map(separated_pair(expr, char('+'), expr), |(expr1, expr2)| {
        Expr::Binary(BinaryOperator::Add, Box::new(expr1), Box::new(expr2))
    })(input)
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

