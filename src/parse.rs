use nom::{
    branch::alt,
    bytes::complete::is_not,
    character::complete::{char, digit1},
    combinator::{map, map_res, recognize},
    number::complete::double,
    sequence::delimited,
    IResult,
};

use crate::{
    expr::Expr,
    value::{Number, Value},
};

pub fn parse(input: &str) -> Expr {
    let (remaining, expr) = expr(input).unwrap();
    assert!(remaining.is_empty());
    expr
}

fn expr(input: &str) -> IResult<&str, Expr> {
    map(value, |value| Expr::Value(value))(input)
}

fn value(input: &str) -> IResult<&str, Value> {
    alt((number_value, string_value))(input)
}

fn number_value(input: &str) -> IResult<&str, Value> {
    map(
        alt((
            map(map_res(recognize(digit1), str::parse), Number::Int),
            map(double, Number::Float),
        )),
        Value::Number,
    )(input)
}

fn string_value(input: &str) -> IResult<&str, Value> {
    map(
        alt((
            delimited(char('"'), is_not("\""), char('"')),
            delimited(char('\''), is_not("'"), char('\'')),
        )),
        |value: &str| Value::String(value.to_string()),
    )(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_parse_integer() {
        assert_eq!(parse("15"), Expr::Value(Value::Number(Number::Int(15))));
    }

    #[test]
    #[ignore]
    fn should_parse_negative_integer() {
        assert_eq!(parse("-6"), Expr::Value(Value::Number(Number::Int(-6))))
    }

    #[test]
    #[ignore]
    fn should_parse_float() {
        assert_eq!(parse("38e-1"), Expr::Value(Value::Number(Number::Float(3.8))))
    }

    #[test]
    fn should_parse_double_quoted_string() {
        assert_eq!(
            parse("\"string value\""),
            Expr::Value(Value::String("string value".to_string()))
        );
    }

    #[test]
    fn should_ignore_single_quotes_inside_double_quoted_string() {
        assert_eq!(
            parse("\"string 'value'\""),
            Expr::Value(Value::String("string 'value'".to_string()))
        );
    }

    #[test]
    fn should_parse_single_quoted_string() {
        assert_eq!(
            parse("'string value'"),
            Expr::Value(Value::String("string value".to_string()))
        );
    }

    #[test]
    fn should_ignore_double_quotes_inside_single_quoted_string() {
        assert_eq!(
            parse("'string \"value\"'"),
            Expr::Value(Value::String("string \"value\"".to_string()))
        );
    }
}
