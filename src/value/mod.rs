#[cfg(feature = "value_serializer")]
pub mod ser;

#[cfg(feature = "value_serializer")]
mod string_ser;

use std::collections::hash_map::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    String(String),
    Int(i128),
    Float(f64),
    Bool(bool),
    Map(HashMap<String, Value>),
    Vec(Vec<Value>),
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

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Float(value)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}
