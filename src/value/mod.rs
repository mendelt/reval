//! The `Value` type encodes data that can be passed in or out from expressions
pub mod convert;
pub mod ser;

use chrono::{prelude::*, TimeDelta};
use itertools::Itertools;
use rust_decimal::Decimal;
use std::{collections::BTreeMap, fmt::Display};

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    String(String),
    Int(i128),
    Float(f64),
    Decimal(Decimal),
    Bool(bool),
    DateTime(DateTime<Utc>),
    Duration(TimeDelta),
    Vec(Vec<Value>),
    Map(BTreeMap<String, Value>),
    None,
}

impl Display for Value {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(value) => write!(formatter, "\"{value}\""),
            Value::Int(value) => write!(formatter, "i{value}"),
            Value::Float(value) => write!(formatter, "f{value}"),
            Value::Decimal(value) => write!(formatter, "d{value}"),
            Value::Bool(value) => write!(formatter, "{value}"),
            Value::DateTime(value) => write!(formatter, "{value}"),
            Value::Duration(value) => write!(formatter, "{value}"),
            Value::Vec(values) => {
                write!(
                    formatter,
                    "[{}]",
                    values.iter().map(ToString::to_string).join(", ")
                )
            }
            Value::Map(map) => write!(
                formatter,
                "{{{}}}",
                map.iter()
                    .map(|(key, value)| format!("{key}: {value}"))
                    .join(", ")
            ),
            Value::None => write!(formatter, "none"),
        }
    }
}

#[cfg(test)]
mod when_displaying_value {
    use super::*;

    #[test]
    fn should_display_string() {
        assert_eq!(Value::String("test".to_string()).to_string(), "\"test\"")
    }

    #[test]
    fn should_display_escaped_string() {
        assert_eq!(
            Value::String("test\n\ttest".to_string()).to_string(),
            "\"test\n\ttest\""
        );
    }

    #[test]
    fn should_display_int() {
        assert_eq!(Value::Int(5).to_string(), "i5");
    }

    #[test]
    fn should_display_float() {
        assert_eq!(Value::Float(5.4).to_string(), "f5.4");
    }

    #[test]
    fn should_display_decimal() {
        assert_eq!(Value::Decimal(Decimal::new(53, 1)).to_string(), "d5.3");
    }

    #[test]
    fn should_display_bool() {
        assert_eq!(Value::Bool(true).to_string(), "true");
        assert_eq!(Value::Bool(false).to_string(), "false");
    }

    #[test]
    fn should_display_none() {
        assert_eq!(Value::None.to_string(), "none");
    }

    #[test]
    fn should_display_datetime() {
        assert_eq!(
            Value::DateTime(Utc.with_ymd_and_hms(2015, 7, 30, 3, 26, 13).unwrap()).to_string(),
            "2015-07-30 03:26:13 UTC"
        );
    }

    #[test]
    fn should_display_duration() {
        assert_eq!(Value::Duration(TimeDelta::days(4)).to_string(), "PT345600S");
    }
}
