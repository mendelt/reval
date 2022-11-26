//! The `Value` type encodes data that can be passed in or out from expressions

#[cfg(feature = "value_serializer")]
pub mod ser;

#[cfg(feature = "value_serializer")]
mod string_ser;

use crate::Error;
use std::collections::hash_map::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    String(String),
    Int(i128),
    Float(f64),
    Bool(bool),
    Map(HashMap<String, Value>),
    Vec(Vec<Value>),
    None,
}

impl Value {
    pub fn new<T: Into<Value>>(value: T) -> Self {
        value.into()
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(value)
    }
}

impl TryFrom<Value> for String {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::String(value) => Ok(value),
            _ => Err(Error::UnexpectedValueType(
                value,
                "Value::String".to_owned(),
            )),
        }
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::String(value.to_string())
    }
}

impl From<i128> for Value {
    fn from(value: i128) -> Self {
        Value::Int(value)
    }
}

impl TryFrom<Value> for i128 {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Int(value) => Ok(value),
            _ => Err(Error::UnexpectedValueType(value, "Value::Int".to_owned())),
        }
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Float(value)
    }
}

impl TryFrom<Value> for f64 {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Float(value) => Ok(value),
            _ => Err(Error::UnexpectedValueType(value, "Value::Float".to_owned())),
        }
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}

impl TryFrom<Value> for bool {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Bool(value) => Ok(value),
            _ => Err(Error::UnexpectedValueType(value, "Value::Bool".to_owned())),
        }
    }
}

impl From<Option<Value>> for Value {
    fn from(option: Option<Value>) -> Self {
        match option {
            Some(value) => value,
            None => Value::None,
        }
    }
}
