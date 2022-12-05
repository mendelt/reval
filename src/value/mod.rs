//! The `Value` type encodes data that can be passed in or out from expressions
pub mod convert;
#[cfg(feature = "value_serializer")]
pub mod ser;

#[cfg(feature = "value_serializer")]
mod string_ser;

use rust_decimal::Decimal;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    String(String),
    Int(i128),
    Float(f64),
    Decimal(Decimal),
    Bool(bool),
    Map(HashMap<String, Value>),
    Vec(Vec<Value>),
    None,
}
