use crate::value::Value;
use nom::{
    branch::alt,
    bytes::complete::is_not,
    character::complete::{char, digit1},
    combinator::{map, map_res, recognize},
    number::complete::double,
    sequence::delimited,
    IResult,
};

pub fn value(input: &str) -> IResult<&str, Value> {
    alt((int_value, string_value))(input)
}

fn int_value(input: &str) -> IResult<&str, Value> {
    alt((
        map(map_res(recognize(digit1), str::parse), Value::Int),
        map(double, Value::Float),
    ))(input)
}

#[cfg(test)]
mod when_parsing_integers {
    use super::*;

    #[test]
    fn should_parse_integer() {
        assert_eq!(int_value("15").unwrap().1, Value::Int(15));
    }

    #[test]
    #[ignore]
    fn should_parse_negative_integer() {
        assert_eq!(int_value("-6").unwrap().1, Value::Int(-6))
    }

    #[test]
    #[ignore]
    fn should_parse_float() {
        // assert_eq!(parse("38e-1"), Expr::Value(Value::Float(3.8)))
    }
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
mod when_parsing_string_value {
    use super::*;

    #[test]
    fn should_parse_double_quoted_string() {
        assert_eq!(
            string_value("\"string value\"").unwrap().1,
            Value::String("string value".to_string())
        );
    }

    #[test]
    fn should_ignore_single_quotes_inside_double_quoted_string() {
        assert_eq!(
            string_value("\"string 'value'\"").unwrap().1,
            Value::String("string 'value'".to_string())
        );
    }

    #[test]
    fn should_parse_single_quoted_string() {
        assert_eq!(
            string_value("'string value'").unwrap().1,
            Value::String("string value".to_string())
        );
    }

    #[test]
    fn should_ignore_double_quotes_inside_single_quoted_string() {
        assert_eq!(
            string_value("'string \"value\"'").unwrap().1,
            Value::String("string \"value\"".to_string())
        );
    }
}
