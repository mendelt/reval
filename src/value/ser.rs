use crate::{
    value::{string_ser::StringSerializer, Value},
    Error, Result,
};
use serde::{
    ser::{
        Error as SerError, SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant,
        SerializeTuple, SerializeTupleStruct, SerializeTupleVariant,
    },
    Serialize, Serializer,
};
use std::{collections::HashMap, fmt::Display};

pub struct ValueSerializer;

impl SerError for Error {
    fn custom<T: Display>(_msg: T) -> Self {
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

    fn serialize_bool(self, value: bool) -> Result<Value> {
        Ok(Value::Bool(value))
    }

    fn serialize_i8(self, value: i8) -> Result<Value> {
        self.serialize_i128(value as i128)
    }

    fn serialize_i16(self, value: i16) -> Result<Value> {
        self.serialize_i128(value as i128)
    }

    fn serialize_i32(self, value: i32) -> Result<Value> {
        self.serialize_i128(value as i128)
    }

    fn serialize_i64(self, value: i64) -> Result<Value> {
        self.serialize_i128(value as i128)
    }

    fn serialize_i128(self, value: i128) -> Result<Value> {
        Ok(Value::Int(value))
    }

    fn serialize_u8(self, value: u8) -> Result<Value> {
        self.serialize_i128(value as i128)
    }

    fn serialize_u16(self, value: u16) -> Result<Value> {
        self.serialize_i128(value as i128)
    }

    fn serialize_u32(self, value: u32) -> Result<Value> {
        self.serialize_i128(value as i128)
    }

    fn serialize_u64(self, value: u64) -> Result<Value> {
        self.serialize_i128(value as i128)
    }

    fn serialize_u128(self, value: u128) -> Result<Value> {
        self.serialize_i128(value as i128)
    }

    fn serialize_f32(self, value: f32) -> Result<Value> {
        self.serialize_f64(value as f64)
    }

    fn serialize_f64(self, value: f64) -> Result<Value> {
        Ok(Value::Float(value))
    }

    fn serialize_char(self, value: char) -> Result<Value> {
        Ok(Value::String(value.to_string()))
    }

    fn serialize_str(self, value: &str) -> Result<Value> {
        Ok(Value::String(value.to_string()))
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<Value> {
        Ok(Value::Vec(
            value.iter().map(|&b| Value::Int(b.into())).collect(),
        ))
    }

    fn serialize_none(self) -> Result<Value> {
        self.serialize_unit()
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Value>
    where
        T: serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Value> {
        Ok(Value::None)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Value> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Value> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, value: &T) -> Result<Value>
    where
        T: serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Value>
    where
        T: serde::Serialize,
    {
        let mut values = HashMap::new();
        values.insert(String::from(variant), value.serialize(self)?);
        Ok(Value::Map(values))
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(SerializeVecValue {
            vec: Vec::with_capacity(len.unwrap_or(0)),
        })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Ok(SerializeTupleVariantValue {
            name: String::from(variant),
            vec: Vec::with_capacity(len),
        })
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(SerializeMapValue {
            map: HashMap::new(),
            next_key: None,
        })
    }

    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Ok(SerializeStructVariantValue {
            name: String::from(variant),
            map: HashMap::new(),
        })
    }
}

pub struct SerializeVecValue {
    vec: Vec<Value>,
}

impl SerializeSeq for SerializeVecValue {
    type Ok = Value;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.vec.push(value.serialize(ValueSerializer)?);
        Ok(())
    }

    fn end(self) -> Result<Value> {
        Ok(Value::Vec(self.vec))
    }
}

impl SerializeTuple for SerializeVecValue {
    type Ok = Value;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Value> {
        SerializeSeq::end(self)
    }
}

impl SerializeTupleStruct for SerializeVecValue {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Value> {
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

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.vec.push(value.serialize(ValueSerializer)?);
        Ok(())
    }

    fn end(self) -> Result<Value> {
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

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.next_key = Some(key.serialize(StringSerializer)?);
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
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

    fn end(self) -> Result<Value> {
        Ok(Value::Map(self.map))
    }
}

impl SerializeStruct for SerializeMapValue {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        SerializeMap::serialize_entry(self, key, value)
    }

    fn end(self) -> Result<Value> {
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

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.map
            .insert(String::from(key), value.serialize(ValueSerializer)?);
        Ok(())
    }

    fn end(self) -> Result<Value> {
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
        assert_serialized(true, true.into());
    }

    #[test]
    fn should_serialize_int() {
        assert_serialized(8u8, 8.into());
    }

    #[test]
    fn should_serialize_string() {
        assert_serialized("String val", "String val".into())
    }

    #[test]
    fn should_serialize_char_to_string() {
        assert_serialized('c', Value::String("c".to_owned()))
    }

    #[test]
    fn should_serialize_byte_arrays_to_vec() {
        assert_serialized(&[3u8, 5], Value::Vec(vec![3.into(), 5.into()]))
    }

    #[test]
    fn should_serialize_none_option() {
        let value: Option<String> = None;
        assert_serialized(value, Value::None)
    }

    #[test]
    fn should_serialize_some_option_to_inner_type() {
        assert_serialized(Some("Value".to_owned()), "Value".into())
    }

    #[test]
    fn should_serialize_newtype_struct_to_inner() {
        #[derive(Serialize)]
        struct NewType(String);

        assert_serialized(NewType("Test".to_owned()), "Test".into());
    }

    #[test]
    fn should_serialize_unit_to_none() {
        assert_serialized((), Value::None)
    }

    #[test]
    fn should_serialize_unit_struct_to_none() {
        #[derive(Serialize)]
        struct Nothing;

        assert_serialized(Nothing {}, Value::None)
    }

    #[test]
    fn should_serialize_sequence_to_vec() {
        assert_serialized([-16, 8], Value::Vec(vec![Value::Int(-16), Value::Int(8)]));
    }

    #[test]
    fn should_serialize_tuple_to_vec() {
        assert_serialized(
            (-16, "Test"),
            Value::Vec(vec![Value::Int(-16), Value::String("Test".to_owned())]),
        );
    }

    #[test]
    fn should_serialize_tuple_struct_to_vec() {
        #[derive(Serialize)]
        struct TupleStruct(i16, String);

        assert_serialized(
            TupleStruct(4, "18".to_owned()),
            Value::Vec(vec![4.into(), "18".into()]),
        );
    }

    #[test]
    fn should_serialize_enum_unit_variant_as_str() {
        #[derive(Serialize)]
        enum TestEnum {
            Variant,
        }

        assert_serialized(TestEnum::Variant, "Variant".into());
    }

    #[test]
    fn should_serialize_enum_val_variant() {
        #[derive(Serialize)]
        enum TestEnum {
            Variant(u64),
        }

        assert_serialized(
            TestEnum::Variant(14),
            Value::Map(HashMap::from([("Variant".to_owned(), Value::Int(14))])),
        );
    }

    #[test]
    fn should_serialize_enum_tuple_variant() {
        #[derive(Serialize)]
        enum TestEnum {
            Variant(u64, String),
        }

        assert_serialized(
            TestEnum::Variant(14, "Test".to_owned()),
            Value::Map(HashMap::from([(
                "Variant".to_owned(),
                Value::Vec(vec![Value::Int(14), Value::String("Test".to_owned())]),
            )])),
        );
    }

    #[test]
    fn should_serialize_enum_struct_variant() {
        #[derive(Serialize)]
        enum TestEnum {
            Variant { value: u64 },
        }

        assert_serialized(
            TestEnum::Variant { value: 16 },
            Value::Map(HashMap::from([(
                "Variant".to_owned(),
                Value::Map(HashMap::from([("value".to_owned(), Value::Int(16))])),
            )])),
        );
    }

    #[test]
    fn should_serialize_enum_with_tagged_variants() {
        #[derive(Serialize)]
        #[serde(tag = "type")]
        enum TestEnum {
            Variant { value: u64 },
        }

        assert_serialized(
            TestEnum::Variant { value: 16 },
            Value::Map(HashMap::from([
                ("type".to_owned(), Value::String("Variant".to_owned())),
                ("value".to_owned(), Value::Int(16)),
            ])),
        );
    }

    #[test]
    fn should_serialize_map() {
        assert_serialized(
            BTreeMap::from([("key", "value")]),
            Value::Map(HashMap::from([(
                "key".to_owned(),
                Value::String("value".to_owned()),
            )])),
        );
    }

    #[test]
    fn should_serialize_struct_to_map() {
        #[derive(Serialize)]
        struct Data {
            age: u16,
            name: String,
        }

        assert_serialized(
            Data {
                age: 21,
                name: "Frank".to_owned(),
            },
            Value::Map(HashMap::from([
                ("age".to_owned(), Value::Int(21)),
                ("name".to_owned(), Value::String("Frank".to_owned())),
            ])),
        )
    }

    /// Helper method that tests if serializing a value yields the expected result
    fn assert_serialized(value: impl Serialize, expected: Value) {
        assert_eq!(value.serialize(ValueSerializer).unwrap(), expected);
    }
}
