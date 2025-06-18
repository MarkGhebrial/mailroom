//! This file was an experiment. A failed one? Probably.
//! 
//! It's a candidate for deletion.

use std::{error::Error, fmt::Display};

use serde::{
    ser::{
        SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
        SerializeTupleStruct, SerializeTupleVariant, Serializer,
    },
    Serialize,
};

use crate::config::Config;

#[derive(Debug)]
struct RatatuiSerializerError;

impl serde::ser::Error for RatatuiSerializerError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        RatatuiSerializerError
    }
}

impl Display for RatatuiSerializerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ratatui serializer error")
    }
}

impl Error for RatatuiSerializerError {}

struct RatatuiSerializer {}

impl Serializer for &mut RatatuiSerializer {
    type Ok = ();

    type Error = RatatuiSerializerError;

    type SerializeSeq = RatatuiListEditor;

    type SerializeTuple = RatatuiListEditor;

    type SerializeTupleStruct = RatatuiListEditor;

    type SerializeTupleVariant = RatatuiListEditor;

    type SerializeMap = RatatuiListEditor;

    type SerializeStruct = RatatuiListEditor;

    type SerializeStructVariant = RatatuiListEditor;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        println!("Serializing bool: {v}");
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        println!("Serializing i8: {v}");
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        println!("Serializing i16: {v}");
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        println!("Serializing i32: {v}");
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        println!("Serializing i64: {v}");
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        println!("Serializing u8: {v}");
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        println!("Serializing u16: {v}");
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        println!("Serializing u32: {v}");
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        println!("Serializing u64: {v}");
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        println!("Serializing f32: {v}");
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        println!("Serializing f64: {v}");
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        println!("Serializing char: {v}");
        Ok(())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        println!("Serializing str: {v}");
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        println!("Serializing bytes: {v:?}");
        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        println!("Serializing none");
        Ok(())
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        println!("Serializing some");
        Ok(())
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        println!("Serializing unit ()");
        Ok(())
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        println!("Serializing unit struct: {name}");
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        println!("Serializing newtype struct");
        Ok(())
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        println!("Serializing newtype variant");
        Ok(())
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(RatatuiListEditor)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(RatatuiListEditor)
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(RatatuiListEditor)
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(RatatuiListEditor)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(RatatuiListEditor)
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(RatatuiListEditor)
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(RatatuiListEditor)
    }
}

struct RatatuiListEditor;
impl SerializeSeq for RatatuiListEditor {
    type Ok = ();

    type Error = RatatuiSerializerError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl SerializeTuple for RatatuiListEditor {
    type Ok = ();

    type Error = RatatuiSerializerError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl SerializeTupleStruct for RatatuiListEditor {
    type Ok = ();

    type Error = RatatuiSerializerError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl SerializeTupleVariant for RatatuiListEditor {
    type Ok = ();

    type Error = RatatuiSerializerError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl SerializeMap for RatatuiListEditor {
    type Ok = ();

    type Error = RatatuiSerializerError;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        todo!()
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl SerializeStruct for RatatuiListEditor {
    type Ok = ();

    type Error = RatatuiSerializerError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl SerializeStructVariant for RatatuiListEditor {
    type Ok = ();

    type Error = RatatuiSerializerError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

#[test]
fn test_serializer() {
    let config: Config = toml::from_str(
        std::fs::read_to_string("config.toml")
            .expect(&format!("Couldn't find config file at config.toml"))
            .as_str(),
    )
    .expect("Invalid configuration");

    config.serialize(&mut RatatuiSerializer {}).unwrap()
}
