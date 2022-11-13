use crate::value::Value;
use displaydoc::Display;
use serde::{
    ser::{
        SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
        SerializeTupleStruct, SerializeTupleVariant,
    },
    Serialize, Serializer,
};
use std::collections::HashMap;

use super::string_ser::StringSerializer;

pub struct ValueSerializer;

#[derive(Debug, Display, thiserror::Error)]
pub enum Error {
    /// An error occured serializing a value
    SerializationError,
}

impl serde::ser::Error for Error {
    fn custom<T>(_msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        todo!()
    }
}

impl Serializer for ValueSerializer {
    type Ok = Value;
    type Error = Error;

    type SerializeSeq = SerializeVecValue;
    type SerializeTuple = SerializeVecValue;
    type SerializeTupleStruct = SerializeVecValue;
    type SerializeTupleVariant = SerializeTupleVariantValue;
    type SerializeMap = SerializeMapValue;
    type SerializeStruct = SerializeMapValue;
    type SerializeStructVariant = SerializeStructVariantValue;

    fn serialize_bool(self, value: bool) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Bool(value))
    }

    fn serialize_i8(self, value: i8) -> Result<Self::Ok, Self::Error> {
        self.serialize_i128(value as i128)
    }

    fn serialize_i16(self, value: i16) -> Result<Self::Ok, Self::Error> {
        self.serialize_i128(value as i128)
    }

    fn serialize_i32(self, value: i32) -> Result<Self::Ok, Self::Error> {
        self.serialize_i128(value as i128)
    }

    fn serialize_i64(self, value: i64) -> Result<Self::Ok, Self::Error> {
        self.serialize_i128(value as i128)
    }

    fn serialize_i128(self, value: i128) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Int(value))
    }

    fn serialize_u8(self, value: u8) -> Result<Self::Ok, Self::Error> {
        self.serialize_i128(value as i128)
    }

    fn serialize_u16(self, value: u16) -> Result<Self::Ok, Self::Error> {
        self.serialize_i128(value as i128)
    }

    fn serialize_u32(self, value: u32) -> Result<Self::Ok, Self::Error> {
        self.serialize_i128(value as i128)
    }

    fn serialize_u64(self, value: u64) -> Result<Self::Ok, Self::Error> {
        self.serialize_i128(value as i128)
    }

    fn serialize_u128(self, value: u128) -> Result<Self::Ok, Self::Error> {
        self.serialize_i128(value as i128)
    }

    fn serialize_f32(self, value: f32) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(value as f64)
    }

    fn serialize_f64(self, value: f64) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Float(value))
    }

    fn serialize_char(self, value: char) -> Result<Self::Ok, Self::Error> {
        Ok(Value::String(value.to_string()))
    }

    fn serialize_str(self, value: &str) -> Result<Self::Ok, Self::Error> {
        Ok(Value::String(value.to_string()))
    }

    fn serialize_bytes(self, _value: &[u8]) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::None)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(SerializeVecValue {
            vec: Vec::with_capacity(len.unwrap_or(0)),
        })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        todo!()
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(SerializeMapValue {
            map: HashMap::new(),
            next_key: None,
        })
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        todo!()
    }
}

pub struct SerializeVecValue {
    vec: Vec<Value>,
}

impl SerializeSeq for SerializeVecValue {
    type Ok = Value;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        self.vec.push(value.serialize(ValueSerializer)?);
        Ok(())
    }

    fn end(self) -> Result<Value, Error> {
        Ok(Value::Vec(self.vec))
    }
}

impl SerializeTuple for SerializeVecValue {
    type Ok = Value;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Value, Error> {
        SerializeSeq::end(self)
    }
}

impl SerializeTupleStruct for SerializeVecValue {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Value, Error> {
        SerializeSeq::end(self)
    }
}

pub struct SerializeTupleVariantValue {
    name: String,
    vec: Vec<Value>,
}

impl SerializeTupleVariant for SerializeTupleVariantValue {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        self.vec.push(value.serialize(ValueSerializer)?);
        Ok(())
    }

    fn end(self) -> Result<Value, Error> {
        let mut map = HashMap::new();

        map.insert(self.name, Value::Vec(self.vec));

        Ok(Value::Map(map))
    }
}

pub struct SerializeMapValue {
    map: HashMap<String, Value>,
    next_key: Option<String>,
}

impl SerializeMap for SerializeMapValue {
    type Ok = Value;
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        self.next_key = Some(key.serialize(StringSerializer)?);
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        let key = self.next_key.take();
        // Panic because this indicates a bug in the program rather than an
        // expected failure.
        let key = key.expect("serialize_value called before serialize_key");
        self.map.insert(key, value.serialize(ValueSerializer)?);
        Ok(())
    }

    fn end(self) -> Result<Value, Error> {
        Ok(Value::Map(self.map))
    }
}

impl SerializeStruct for SerializeMapValue {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        SerializeMap::serialize_entry(self, key, value)
    }

    fn end(self) -> Result<Value, Error> {
        SerializeMap::end(self)
    }
}

pub struct SerializeStructVariantValue {
    name: String,
    map: HashMap<String, Value>,
}

impl SerializeStructVariant for SerializeStructVariantValue {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        self.map
            .insert(String::from(key), value.serialize(ValueSerializer)?);
        Ok(())
    }

    fn end(self) -> Result<Value, Error> {
        let mut map = HashMap::new();

        map.insert(self.name, Value::Map(self.map));

        Ok(Value::Map(map))
    }
}

#[cfg(test)]
mod when_serializing_to_value {
    use std::collections::BTreeMap;

    use super::*;
    use serde::Serialize;

    #[test]
    fn should_serialize_bool() {
        assert_eq!(true.serialize(ValueSerializer).unwrap(), true.into());
    }

    #[test]
    fn should_serialize_int() {
        assert_eq!(8u8.serialize(ValueSerializer).unwrap(), 8.into());
    }

    #[test]
    fn should_serialize_string() {
        assert_eq!(
            "String val".serialize(ValueSerializer).unwrap(),
            "String val".into()
        )
    }

    #[test]
    fn should_serialize_char_to_string() {
        assert_eq!(
            'c'.serialize(ValueSerializer).unwrap(),
            Value::String("c".to_owned())
        )
    }

    #[test]
    fn should_serialize_none_option_as_none() {
        let value: Option<String> = None;
        assert_eq!(value.serialize(ValueSerializer).unwrap(), Value::None)
    }

    #[test]
    fn should_serialize_some_option_as_inner_type() {
        assert_eq!(
            Some("Value".to_owned()).serialize(ValueSerializer).unwrap(),
            "Value".into()
        )
    }

    #[test]
    fn should_serialize_unit_to_none() {
        assert_eq!(().serialize(ValueSerializer).unwrap(), Value::None)
    }

    #[test]
    fn should_serialize_unit_struct_to_none() {
        #[derive(Serialize)]
        struct Nothing;

        assert_eq!(Nothing {}.serialize(ValueSerializer).unwrap(), Value::None)
    }

    #[test]
    fn should_serialize_sequence_to_vec() {
        assert_eq!(
            [-16, 8].serialize(ValueSerializer).unwrap(),
            Value::Vec(vec![Value::Int(-16), Value::Int(8)])
        );
    }

    #[test]
    fn should_serialize_tuple_to_vec() {
        assert_eq!(
            (-16, "Test").serialize(ValueSerializer).unwrap(),
            Value::Vec(vec![Value::Int(-16), Value::String("Test".to_owned())])
        );
    }

    #[test]
    fn should_serialize_map() {
        assert_eq!(
            BTreeMap::from([("key", "value")])
                .serialize(ValueSerializer)
                .unwrap(),
            Value::Map(HashMap::from([(
                "key".to_owned(),
                Value::String("value".to_owned())
            )]))
        );
    }

    #[test]
    fn should_serialize_struct_to_map() {
        #[derive(Serialize)]
        struct Data {
            age: u16,
            name: String,
        }

        assert_eq!(
            Data {
                age: 21,
                name: "Frank".to_owned()
            }
            .serialize(ValueSerializer)
            .unwrap(),
            Value::Map(HashMap::from([
                ("age".to_owned(), Value::Int(21)),
                ("name".to_owned(), Value::String("Frank".to_owned())),
            ]))
        )
    }
}
