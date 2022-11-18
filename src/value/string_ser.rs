use crate::{Error, Result};
use serde::{ser::Impossible, Serializer};

/// Serializer that only serializes String values, this can be useful for
/// serializing map keys. Will raise an error when serializing anything other
/// than a String
pub struct StringSerializer;

fn not_a_string<T>() -> Result<T> {
    Err(Error::ser("String type expected"))
}

impl Serializer for StringSerializer {
    type Ok = String;
    type Error = Error;

    type SerializeSeq = Impossible<String, Error>;
    type SerializeTuple = Impossible<String, Error>;
    type SerializeTupleStruct = Impossible<String, Error>;
    type SerializeTupleVariant = Impossible<String, Error>;
    type SerializeMap = Impossible<String, Error>;
    type SerializeStruct = Impossible<String, Error>;
    type SerializeStructVariant = Impossible<String, Error>;

    fn serialize_bool(self, _: bool) -> Result<String> {
        not_a_string()
    }

    fn serialize_i8(self, _: i8) -> Result<String> {
        not_a_string()
    }

    fn serialize_i16(self, _: i16) -> Result<String> {
        not_a_string()
    }

    fn serialize_i32(self, _: i32) -> Result<String> {
        not_a_string()
    }

    fn serialize_i64(self, _: i64) -> Result<String> {
        not_a_string()
    }

    fn serialize_u8(self, _: u8) -> Result<String> {
        not_a_string()
    }

    fn serialize_u16(self, _: u16) -> Result<String> {
        not_a_string()
    }

    fn serialize_u32(self, _: u32) -> Result<String> {
        not_a_string()
    }

    fn serialize_u64(self, _: u64) -> Result<String> {
        not_a_string()
    }

    fn serialize_f32(self, _: f32) -> Result<String> {
        not_a_string()
    }

    fn serialize_f64(self, _: f64) -> Result<String> {
        not_a_string()
    }

    fn serialize_char(self, _: char) -> Result<String> {
        not_a_string()
    }

    fn serialize_str(self, value: &str) -> Result<String> {
        Ok(value.to_string())
    }

    fn serialize_bytes(self, _: &[u8]) -> Result<String> {
        not_a_string()
    }

    fn serialize_none(self) -> Result<String> {
        not_a_string()
    }

    fn serialize_some<T: ?Sized>(self, _: &T) -> Result<String>
    where
        T: serde::Serialize,
    {
        not_a_string()
    }

    fn serialize_unit(self) -> Result<String> {
        not_a_string()
    }

    fn serialize_unit_struct(self, _: &'static str) -> Result<String> {
        not_a_string()
    }

    fn serialize_unit_variant(self, _: &'static str, _: u32, _: &'static str) -> Result<String> {
        not_a_string()
    }

    fn serialize_newtype_struct<T: ?Sized>(self, _: &'static str, _: &T) -> Result<String>
    where
        T: serde::Serialize,
    {
        not_a_string()
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: &T,
    ) -> Result<String>
    where
        T: serde::Serialize,
    {
        not_a_string()
    }

    fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq> {
        not_a_string()
    }

    fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple> {
        not_a_string()
    }

    fn serialize_tuple_struct(
        self,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        not_a_string()
    }

    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        not_a_string()
    }

    fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap> {
        not_a_string()
    }

    fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeStruct> {
        not_a_string()
    }

    fn serialize_struct_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStructVariant> {
        not_a_string()
    }
}
