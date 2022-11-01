use serde::{Serializer, ser::Impossible};

use super::ser::Error;

pub struct StringSerializer;

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

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Err(Error::SerializationError)
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Err(Error::SerializationError)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Err(Error::SerializationError)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Err(Error::SerializationError)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Err(Error::SerializationError)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Err(Error::SerializationError)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Err(Error::SerializationError)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Err(Error::SerializationError)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Err(Error::SerializationError)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Err(Error::SerializationError)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Err(Error::SerializationError)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Err(Error::SerializationError)
    }

    fn serialize_str(self, value: &str) -> Result<Self::Ok, Self::Error> {
        Ok(value.to_string())
    }

    fn serialize_bytes(self, _: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(Error::SerializationError)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::SerializationError)
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize 
    {
        Err(Error::SerializationError)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::SerializationError)
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(Error::SerializationError)
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(Error::SerializationError)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize
    {
        Err(Error::SerializationError)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize 
    {
        Err(Error::SerializationError)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(Error::SerializationError)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(Error::SerializationError)
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(Error::SerializationError)
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(Error::SerializationError)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(Error::SerializationError)
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(Error::SerializationError)
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(Error::SerializationError)
    }
}