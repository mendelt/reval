use crate::value::Value;
use serde::de::{Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};
use std::{collections::BTreeMap, fmt};

impl<'de> Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(ValueVisitor)
    }
}

struct ValueVisitor;

impl<'de> Visitor<'de> for ValueVisitor {
    type Value = Value;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("any valid reval value")
    }

    fn visit_bool<E>(self, value: bool) -> Result<Value, E> {
        Ok(value.into())
    }

    fn visit_i64<E>(self, value: i64) -> Result<Value, E> {
        Ok(value.into())
    }

    fn visit_u64<E>(self, value: u64) -> Result<Value, E> {
        Ok(value.into())
    }

    fn visit_f64<E>(self, value: f64) -> Result<Value, E> {
        Ok(value.into())
    }

    fn visit_str<E>(self, value: &str) -> Result<Value, E> {
        Ok(value.into())
    }

    fn visit_string<E>(self, value: String) -> Result<Value, E> {
        Ok(value.into())
    }

    fn visit_unit<E>(self) -> Result<Value, E> {
        Ok(Value::None)
    }

    fn visit_none<E>(self) -> Result<Value, E> {
        Ok(Value::None)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer)
    }

    fn visit_seq<V>(self, mut visitor: V) -> Result<Value, V::Error>
    where
        V: SeqAccess<'de>,
    {
        let mut vec = Vec::new();

        while let Some(elem) = visitor.next_element()? {
            vec.push(elem);
        }

        Ok(Value::Vec(vec))
    }

    fn visit_map<V>(self, mut visitor: V) -> Result<Value, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut values = BTreeMap::new();

        while let Some((key, value)) = visitor.next_entry()? {
            values.insert(key, value);
        }

        Ok(Value::Map(values))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("1", Value::Int(1))]
    #[case("-2", Value::Int(-2))]
    #[case("true", Value::Bool(true))]
    #[case("false", Value::Bool(false))]
    #[case("\"string\"", Value::String("string".to_string()))]
    #[case("[1, 2, 3]", Value::Vec(vec![Value::Int(1), Value::Int(2), Value::Int(3)]))]
    #[case("{\"one\": 1, \"two\": 2}", Value::Map([("one".to_string(), Value::Int(1)), ("two".to_string(), Value::Int(2))].into()))]
    fn should_deserialize(#[case] from: &str, #[case] into: Value) {
        assert_eq!(serde_json::from_str::<Value>(from).unwrap(), into);
    }
}
