//! Easy conversions to and from `Value` for a set of primitive types that have
//! logical Value representations or conversions to them

use super::Value;
use crate::Error;
use rust_decimal::Decimal;
use std::collections::{BTreeMap, HashMap};

// Convert to and from Value::String

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

// Convert integer types to and from Value::Int

impl From<i128> for Value {
    fn from(value: i128) -> Self {
        Value::Int(value)
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        (i128::from(value)).into()
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        (i128::from(value)).into()
    }
}

impl From<i16> for Value {
    fn from(value: i16) -> Self {
        (i128::from(value)).into()
    }
}

impl From<i8> for Value {
    fn from(value: i8) -> Self {
        (i128::from(value)).into()
    }
}

impl From<u64> for Value {
    fn from(value: u64) -> Self {
        (i128::from(value)).into()
    }
}

impl From<u32> for Value {
    fn from(value: u32) -> Self {
        (i128::from(value)).into()
    }
}

impl From<u16> for Value {
    fn from(value: u16) -> Self {
        (i128::from(value)).into()
    }
}

impl From<u8> for Value {
    fn from(value: u8) -> Self {
        (i128::from(value)).into()
    }
}

impl From<usize> for Value {
    fn from(value: usize) -> Self {
        (value as i128).into()
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

impl TryFrom<Value> for i64 {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        Ok(i128::try_from(value)?.try_into()?)
    }
}

impl TryFrom<Value> for i32 {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        Ok(i128::try_from(value)?.try_into()?)
    }
}

impl TryFrom<Value> for i16 {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        Ok(i128::try_from(value)?.try_into()?)
    }
}

impl TryFrom<Value> for i8 {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        Ok(i128::try_from(value)?.try_into()?)
    }
}

impl TryFrom<Value> for u128 {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        Ok(i128::try_from(value)?.try_into()?)
    }
}

impl TryFrom<Value> for u64 {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        Ok(i128::try_from(value)?.try_into()?)
    }
}

impl TryFrom<Value> for u32 {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        Ok(i128::try_from(value)?.try_into()?)
    }
}

impl TryFrom<Value> for u16 {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        Ok(i128::try_from(value)?.try_into()?)
    }
}

impl TryFrom<Value> for u8 {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        Ok(i128::try_from(value)?.try_into()?)
    }
}

// Convert to and from Value::Float

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Float(value)
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        (f64::from(value)).into()
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

// Convert to and from Value::Decimal

impl From<Decimal> for Value {
    fn from(value: Decimal) -> Self {
        Value::Decimal(value)
    }
}

impl TryFrom<Value> for Decimal {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Decimal(value) => Ok(value),
            _ => Err(Error::UnexpectedValueType(
                value,
                "Value::Decimal".to_owned(),
            )),
        }
    }
}

// Convert to and from Value::Bool

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

// Convert option values to Value

impl From<Option<Value>> for Value {
    fn from(option: Option<Value>) -> Self {
        match option {
            Some(value) => value,
            None => Value::None,
        }
    }
}

// Convert to and from Value::Map

impl<K: Into<String>, V: Into<Value>> From<HashMap<K, V>> for Value {
    fn from(map: HashMap<K, V>) -> Self {
        Value::Map(
            map.into_iter()
                .map(|(key, value)| (key.into(), value.into()))
                .collect(),
        )
    }
}

impl<K: Into<String>, V: Into<Value>> From<BTreeMap<K, V>> for Value {
    fn from(map: BTreeMap<K, V>) -> Self {
        Value::Map(
            map.into_iter()
                .map(|(key, value)| (key.into(), value.into()))
                .collect(),
        )
    }
}

impl TryFrom<Value> for HashMap<String, Value> {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Map(map) => Ok(map),
            _ => Err(Error::UnexpectedValueType(value, "Value::Map".to_owned())),
        }
    }
}

impl TryFrom<Value> for BTreeMap<String, Value> {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Map(map) => Ok(map.into_iter().collect()),
            _ => Err(Error::UnexpectedValueType(value, "Value::Map".to_owned())),
        }
    }
}

impl<V: TryFrom<Value, Error = Error>> TryFrom<Value> for HashMap<String, V> {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Map(map) => map
                .into_iter()
                .map(|(key, val)| val.try_into().map(|val| (key, val)))
                .collect(),
            _ => Err(Error::UnexpectedValueType(value, "Value::Map".to_owned())),
        }
    }
}

impl<V: TryFrom<Value, Error = Error>> TryFrom<Value> for BTreeMap<String, V> {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Map(map) => map
                .into_iter()
                .map(|(key, val)| val.try_into().map(|val| (key, val)))
                .collect(),
            _ => Err(Error::UnexpectedValueType(value, "Value::Map".to_owned())),
        }
    }
}

// Convert to and from Value::Vec

impl<V: Into<Value>> From<Vec<V>> for Value {
    fn from(vec: Vec<V>) -> Self {
        Value::Vec(vec.into_iter().map(|value| value.into()).collect())
    }
}

impl<V: TryFrom<Value, Error = Error>> TryFrom<Value> for Vec<V> {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Vec(vec) => vec.into_iter().map(|val| val.try_into()).collect(),
            _ => Err(Error::UnexpectedValueType(value, "Value::Vec".to_owned())),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_convert_map_directly_to_string_value_hashmap() {
        let value: Value = HashMap::from([("Key 1", 25), ("Key 2", 24), ("Key 3", 12)]).into();

        // Convert the value back into a different type of map
        let new_map: HashMap<String, Value> = value.try_into().unwrap();

        assert_eq!(
            new_map,
            HashMap::from([
                ("Key 1".to_string(), 25.into()),
                ("Key 2".to_string(), 24.into()),
                ("Key 3".to_string(), 12.into())
            ])
        );
    }

    #[test]
    fn should_convert_map_to_hashmap() {
        let map = BTreeMap::from([("Key 1", 25), ("Key 2", 24), ("Key 3", 12)]);

        // Convert the map into a value
        let value: Value = map.into();

        // Convert the value back into a different type of map
        let new_map: HashMap<String, u32> = value.try_into().unwrap();

        assert_eq!(
            new_map,
            HashMap::from([
                ("Key 1".to_string(), 25),
                ("Key 2".to_string(), 24),
                ("Key 3".to_string(), 12)
            ])
        );
    }

    #[test]
    fn should_convert_map_to_btreemap() {
        let map = HashMap::from([("Key 1", 25), ("Key 2", 24), ("Key 3", 12)]);

        // Convert the map into a value
        let value: Value = map.into();

        // Convert the value back into a different type of map
        let new_map: BTreeMap<String, u32> = value.try_into().unwrap();

        assert_eq!(
            new_map,
            BTreeMap::from([
                ("Key 1".to_string(), 25),
                ("Key 2".to_string(), 24),
                ("Key 3".to_string(), 12)
            ])
        );
    }

    #[test]
    fn should_convert_value_to_vec() {
        let list = vec!["item 1", "item 2", "item 3"];

        let value: Value = list.into();

        let new_list: Vec<String> = value.try_into().unwrap();

        assert_eq!(
            new_list,
            vec![
                "item 1".to_string(),
                "item 2".to_string(),
                "item 3".to_string()
            ]
        )
    }
}
