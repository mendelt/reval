use std::str::FromStr;

use crate::value::Value;
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{char, digit0, digit1},
    combinator::{map, map_res, opt, recognize},
    sequence::{delimited, pair, tuple},
    IResult,
};

pub fn value(input: &str) -> IResult<&str, Value> {
    alt((
        bool_value,
        alt((float_value, alt((int_value, string_value)))),
    ))(input)
}

fn int_value(input: &str) -> IResult<&str, Value> {
    map(map_res(recognize_int, str::parse), Value::Int)(input)
}

fn recognize_int(input: &str) -> IResult<&str, &str> {
    recognize(map(pair(opt(alt((char('+'), char('-')))), digit1), |_| ()))(input)
}

#[cfg(test)]
mod when_parsing_integer_value {
    use super::*;

    #[test]
    fn should_parse_integer() {
        assert_eq!(value("15").unwrap().1, Value::Int(15));
    }

    #[test]
    fn should_parse_negative_integer() {
        assert_eq!(value("-6").unwrap().1, Value::Int(-6))
    }
}

fn float_value(input: &str) -> IResult<&str, Value> {
    map(map_res(recognize_float, str::parse), Value::Float)(input)
}

fn recognize_float(input: &str) -> IResult<&str, &str> {
    recognize(tuple((
        opt(alt((char('+'), char('-')))),
        alt((
            map(tuple((digit0, char('.'), digit1)), |_| ()),
            map(tuple((char('.'), digit1)), |_| ()),
        )),
        opt(tuple((
            alt((char('e'), char('E'))),
            opt(alt((char('+'), char('-')))),
            digit1,
        ))),
    )))(input)
}

#[cfg(test)]
mod when_parsing_float_value {
    use super::*;

    #[test]
    fn should_parse_simple_float() {
        assert_eq!(value("5.0").unwrap().1, Value::Float(5.0));
    }

    #[test]
    fn should_parse_negative_float() {
        assert_eq!(value("-5.0").unwrap().1, Value::Float(-5.0));
    }

    #[test]
    fn should_parse_exponent() {
        assert_eq!(value("38.0e-1").unwrap().1, Value::Float(3.8))
    }
}

fn bool_value(input: &str) -> IResult<&str, Value> {
    map(
        map_res(alt((tag("true"), tag("false"))), FromStr::from_str),
        Value::Bool,
    )(input)
}

#[cfg(test)]
mod when_parsing_bool_value {
    use super::*;

    #[test]
    fn should_parse_true() {
        assert_eq!(bool_value("true").unwrap().1, Value::Bool(true));
        // assert_eq!(bool_value("true").unwrap().1, Value::Bool(true));
    }

    #[test]
    fn should_parse_false() {
        assert_eq!(bool_value("false").unwrap().1, Value::Bool(false));
    }

    #[test]
    fn should_not_parse_non_bool() {
        assert!(bool_value("stuff").is_err());
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
