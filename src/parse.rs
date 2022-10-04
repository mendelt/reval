use nom::{
    branch::alt, bytes::complete::is_not, character::complete::char, combinator::map,
    sequence::delimited, IResult,
};

use crate::{expr::Expr, value::Value};

pub fn parse(input: &str) -> Expr {
    let (remaining, expr) = expr(input).unwrap();
    assert!(remaining.is_empty());
    expr
}

fn expr(input: &str) -> IResult<&str, Expr> {
    map(value, |value| Expr::Value(value))(input)
}

fn value(input: &str) -> IResult<&str, Value> {
    string_value(input)
}

fn string_value(input: &str) -> IResult<&str, Value> {
    map(alt((double_quoted_string, single_quoted_string)), |value| {
        Value::String(value.to_string())
    })(input)
}

fn double_quoted_string(input: &str) -> IResult<&str, &str> {
    delimited(char('"'), is_not("\""), char('"'))(input)
}

fn single_quoted_string(input: &str) -> IResult<&str, &str> {
    delimited(char('\''), is_not("'"), char('\''))(input)
}

#[cfg(test)]
mod test {
    use super::*;

    // #[test]
    // fn should_parse_integer() {
    //     assert_eq!(parse("0"), Expr::Value(Value::Number(Number::Int(0))));
    // }

    // #[test]
    // fn should_parse_integers() {
    //     assert_eq!(parse("6"), Expr::Value(Value::Number(Number::Int(6))))
    // }

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
