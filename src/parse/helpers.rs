//! Helper methods for the lalrpop parser

use crate::{
    parse::unescape::{unescape, UnescapeError},
    value::Value,
};
use rust_decimal::{self, Decimal};
use std::num::{ParseFloatError, ParseIntError};
use std::str::FromStr;

pub(crate) fn parse_int_value(value: &str) -> Result<Value, ParseIntError> {
    Ok(Value::Int(i128::from_str(&value[1..])?))
}

pub(crate) fn parse_hex_int_value(value: &str) -> Result<Value, ParseIntError> {
    Ok(Value::Int(i128::from_str_radix(&value[2..], 16)?))
}

pub(crate) fn parse_bin_int_value(value: &str) -> Result<Value, ParseIntError> {
    Ok(Value::Int(i128::from_str_radix(&value[2..], 2)?))
}

pub(crate) fn parse_oct_int_value(value: &str) -> Result<Value, ParseIntError> {
    Ok(Value::Int(i128::from_str_radix(&value[2..], 8)?))
}

pub(crate) fn parse_float_value(value: &str) -> Result<Value, ParseFloatError> {
    Ok(Value::Float(f64::from_str(&value[1..])?))
}

pub(crate) fn parse_decimal_value(value: &str) -> Result<Value, rust_decimal::Error> {
    Ok(Value::Decimal(Decimal::from_str(&value[1..])?))
}

pub(crate) fn parse_string_literal(value: &str) -> Result<Value, UnescapeError> {
    let unquoted = &value[1..value.len() - 1];
    let unescaped = unescape(unquoted)?;

    Ok(Value::String(unescaped))
}
