use super::error::{Error, Result};
use itoa;
use serde::{ser, Serialize};
use std::{i64, io, num::FpCategory};

pub struct Serializer<W>
where
    W: io::Write,
{
    writer: W,
}

impl<W> Serializer<W>
where
    W: io::Write,
{
    pub fn new(writer: W) -> Self {
        Serializer { writer: writer }
    }
}

pub fn to_writer<W, T: ?Sized>(writer: W, value: &T) -> Result<()>
where
    W: io::Write,
    T: Serialize,
{
    let mut ser = Serializer::new(writer);
    value.serialize(&mut ser)?;
    Ok(())
}

pub fn to_vec<T: ?Sized>(value: &T) -> Result<Vec<u8>>
where
    T: Serialize,
{
    let mut writer = Vec::with_capacity(128);
    to_writer(&mut writer, value)?;
    Ok(writer)
}

pub fn to_string<T: ?Sized>(value: &T) -> Result<String>
where
    T: Serialize,
{
    let vec = to_vec(value)?;
    let string = unsafe { String::from_utf8_unchecked(vec) };
    Ok(string)
}

impl<'a, W> ser::Serializer for &'a mut Serializer<W>
where
    W: io::Write,
{
    type Ok = ();

    type Error = Error;

    type SerializeSeq = Compound<'a, W>;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<()> {
        let s = if v {
            b"true" as &[u8]
        } else {
            b"false" as &[u8]
        };
        self.writer.write_all(s).map_err(Error::Io)?;
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        itoa::write(&mut self.writer, v).map_err(Error::Io)?;
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        itoa::write(&mut self.writer, v).map_err(Error::Io)?;
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        itoa::write(&mut self.writer, v).map_err(Error::Io)?;
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        itoa::write(&mut self.writer, v).map_err(Error::Io)?;
        Ok(())
    }

    // TODO - Radu M
    // check the serde_if_integer128! macro for 128-bit integers

    fn serialize_u8(self, v: u8) -> Result<()> {
        itoa::write(&mut self.writer, v).map_err(Error::Io)?;
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        itoa::write(&mut self.writer, v).map_err(Error::Io)?;
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        itoa::write(&mut self.writer, v).map_err(Error::Io)?;
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        itoa::write(&mut self.writer, v).map_err(Error::Io)?;
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        match v.classify() {
            FpCategory::Nan | FpCategory::Infinite => {
                return Err(Error::Custom(String::from(format!(
                    "value not allowed in cannonical JSON: {}",
                    v
                ))))
            }
            _ => {
                if v.fract() != 0.0 {
                    return Err(Error::Custom(String::from(format!(
                        "value not allowed in cannonical JSON: {}",
                        v
                    ))));
                }
                if v != (v as i64) as f32 {
                    return Err(Error::Custom(String::from(format!(
                        "value not allowed in cannonical JSON: {}",
                        v
                    ))));
                }
                itoa::write(&mut self.writer, v as i64).map_err(Error::Io)?;
            }
        }
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        match v.classify() {
            FpCategory::Nan | FpCategory::Infinite => {
                return Err(Error::Custom(String::from(format!(
                    "value not allowed in cannonical JSON: {}",
                    v
                ))))
            }
            _ => {
                if v.fract() != 0.0 {
                    return Err(Error::Custom(String::from(format!(
                        "value not allowed in cannonical JSON: {}",
                        v
                    ))));
                }
                if v != (v as i64) as f64 {
                    return Err(Error::Custom(String::from(format!(
                        "value not allowed in cannonical JSON: {}",
                        v
                    ))));
                }
                itoa::write(&mut self.writer, v as i64).map_err(Error::Io)?;
            }
        }
        Ok(())
    }

    // Serialize a char as a single-character string. Other formats may
    // represent this differently.
    fn serialize_char(self, v: char) -> Result<()> {
        escape_char(&mut self.writer, v)?;
        Ok(())
    }

    // This only works for strings that don't require escape sequences but you
    // get the idea. For example it would emit invalid JSON if the input string
    // contains a '"' character.
    fn serialize_str(self, v: &str) -> Result<()> {
        escape_str(&mut self.writer, v)?;
        Ok(())
    }

    // Serialize a byte array as an array of bytes. Could also use a base64
    // string here. Binary formats will typically represent byte arrays more
    // compactly.
    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        use serde::ser::SerializeSeq;
        let mut seq = self.serialize_seq(Some(v.len()))?;
        for byte in v {
            seq.serialize_element(byte)?;
        }
        seq.end()
    }

    // An absent optional is represented as the JSON `null`.
    fn serialize_none(self) -> Result<()> {
        //self.serialize_unit()
        Ok(())
    }

    // A present optional is represented as just the contained value. Note that
    // this is a lossy representation. For example the values `Some(())` and
    // `None` both serialize as just `null`. Unfortunately this is typically
    // what people expect when working with JSON. Other formats are encouraged
    // to behave more intelligently if possible.
    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    // In Serde, unit means an anonymous value containing no data. Map this to
    // JSON as `null`.
    fn serialize_unit(self) -> Result<()> {
        //self.output += "null";
        Ok(())
    }

    // Unit struct means a named value containing no data. Again, since there is
    // no data, map this to JSON as `null`. There is no need to serialize the
    // name in most formats.
    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        //self.serialize_unit()
        Ok(())
    }

    // When serializing a unit variant (or any other kind of variant), formats
    // can choose whether to keep track of it by index or by name. Binary
    // formats typically use the index of the variant and human-readable formats
    // typically use the name.
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        //self.serialize_str(variant)
        Ok(())
    }

    // As is done here, serializers are encouraged to treat newtype structs as
    // insignificant wrappers around the data they contain.
    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    // Note that newtype variant (and all of the other variant serialization
    // methods) refer exclusively to the "externally tagged" enum
    // representation.
    //
    // Serialize this to JSON in externally tagged form as `{ NAME: VALUE }`.
    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        // self.output += "{";
        // variant.serialize(&mut *self)?;
        // self.output += ":";
        // value.serialize(&mut *self)?;
        // self.output += "}";
        Ok(())
    }

    // Now we get to the serialization of compound types.
    //
    // The start of the sequence, each value, and the end are three separate
    // method calls. This one is responsible only for serializing the start,
    // which in JSON is `[`.
    //
    // The length of the sequence may or may not be known ahead of time. This
    // doesn't make a difference in JSON because the length is not represented
    // explicitly in the serialized form. Some serializers may only be able to
    // support sequences for which the length is known up front.
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        if len == Some(0) {
            self.writer.write_all(b"[]")?;
            Ok(Compound::Map {
                ser: self,
                state: State::Empty,
            })
        } else {
            self.writer.write_all(b"[")?;
            Ok(Compound::Map {
                ser: self,
                state: State::First,
            })
        }
    }

    // Tuples look just like sequences in JSON. Some formats may be able to
    // represent tuples more efficiently by omitting the length, since tuple
    // means that the corresponding `Deserialize implementation will know the
    // length without needing to look at the serialized data.
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        // self.serialize_seq(Some(len))
        Ok(self)
    }

    // Tuple structs look just like sequences in JSON.
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        // self.serialize_seq(Some(len))
        Ok(self)
    }

    // Tuple variants are represented in JSON as `{ NAME: [DATA...] }`. Again
    // this method is only responsible for the externally tagged representation.
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        // self.output += "{";
        // variant.serialize(&mut *self)?;
        // self.output += ":[";
        Ok(self)
    }

    // Maps are represented in JSON as `{ K: V, K: V, ... }`.
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        // self.output += "{";
        Ok(self)
    }

    // Structs look just like maps in JSON. In particular, JSON requires that we
    // serialize the field names of the struct. Other formats may be able to
    // omit the field names when serializing structs because the corresponding
    // Deserialize implementation is required to know what the keys are without
    // looking at the serialized data.
    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        // self.serialize_map(Some(len))
        Ok(self)
    }

    // Struct variants are represented in JSON as `{ NAME: { K: V, ... } }`.
    // This is the externally tagged representation.
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        // self.output += "{";
        // variant.serialize(&mut *self)?;
        // self.output += ":{";
        Ok(self)
    }
}

#[derive(Eq, PartialEq)]
#[doc(hidden)]
pub enum State {
    Empty,
    First,
    Rest,
}

#[doc(hidden)]
pub enum Compound<'a, W: 'a>
where
    W: io::Write,
{
    Map {
        ser: &'a mut Serializer<W>,
        state: State,
    },
}

impl<'a, W> ser::SerializeSeq for Compound<'a, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        match *self {
            Compound::Map {
                ref mut ser,
                ref mut state,
            } => {
                // begin array value
                // if the value is not thre first, write a ","
                match *state {
                    State::Rest => ser.writer.write_all(b",")?,
                    _ => (),
                }
                *state = State::Rest;
                value.serialize(&mut **ser)?;

                Ok(())
            }
        }
    }

    fn end(self) -> Result<()> {
        match self {
            Compound::Map { ser, state } => {
                match state {
                    State::Empty => {}
                    _ => ser.writer.write_all(b"]")?,
                }

                Ok(())
            }
        }
    }
}

// Same thing but for tuples.
impl<'a, W> ser::SerializeTuple for &'a mut Serializer<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        // if !self.output.ends_with('[') {
        //     self.output += ",";
        // }
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        // self.output += "]";
        Ok(())
    }
}

// Same thing but for tuple structs.
impl<'a, W> ser::SerializeTupleStruct for &'a mut Serializer<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        // if !self.output.ends_with('[') {
        //     self.output += ",";
        // }
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        // self.output += "]";
        Ok(())
    }
}

// Tuple variants are a little different. Refer back to the
// `serialize_tuple_variant` method above:
//
//    self.output += "{";
//    variant.serialize(&mut *self)?;
//    self.output += ":[";
//
// So the `end` method in this impl is responsible for closing both the `]` and
// the `}`.
impl<'a, W> ser::SerializeTupleVariant for &'a mut Serializer<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        // if !self.output.ends_with('[') {
        //     self.output += ",";
        // }
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        // self.output += "]}";
        Ok(())
    }
}

// Some `Serialize` types are not able to hold a key and value in memory at the
// same time so `SerializeMap` implementations are required to support
// `serialize_key` and `serialize_value` individually.
//
// There is a third optional method on the `SerializeMap` trait. The
// `serialize_entry` method allows serializers to optimize for the case where
// key and value are both available simultaneously. In JSON it doesn't make a
// difference so the default behavior for `serialize_entry` is fine.
impl<'a, W> ser::SerializeMap for &'a mut Serializer<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    // The Serde data model allows map keys to be any serializable type. JSON
    // only allows string keys so the implementation below will produce invalid
    // JSON if the key serializes as something other than a string.
    //
    // A real JSON serializer would need to validate that map keys are strings.
    // This can be done by using a different Serializer to serialize the key
    // (instead of `&mut **self`) and having that other serializer only
    // implement `serialize_str` and return an error on any other data type.
    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        // if !self.output.ends_with('{') {
        //     self.output += ",";
        // }
        key.serialize(&mut **self)
    }

    // It doesn't make a difference whether the colon is printed at the end of
    // `serialize_key` or at the beginning of `serialize_value`. In this case
    // the code is a bit simpler having it here.
    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        // self.output += ":";
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        // self.output += "}";
        Ok(())
    }
}

// Structs are like maps in which the keys are constrained to be compile-time
// constant strings.
impl<'a, W> ser::SerializeStruct for &'a mut Serializer<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        // if !self.output.ends_with('{') {
        //     self.output += ",";
        // }
        // key.serialize(&mut **self)?;
        // self.output += ":";
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        // self.output += "}";
        Ok(())
    }
}

// Similar to `SerializeTupleVariant`, here the `end` method is responsible for
// closing both of the curly braces opened by `serialize_struct_variant`.
impl<'a, W> ser::SerializeStructVariant for &'a mut Serializer<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        // if !self.output.ends_with('{') {
        //     self.output += ",";
        // }
        // key.serialize(&mut **self)?;
        // self.output += ":";
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        // self.output += "}}";
        Ok(())
    }
}

fn escape_char<W: io::Write>(writer: &mut W, v: char) -> Result<()> {
    let mut s = String::new();
    s.push(v);
    escape_str(writer, &s)
}

fn escape_str<W: io::Write>(writer: &mut W, v: &str) -> Result<()> {
    writer.write_all(b"\"")?;

    let bytes = v.as_bytes();
    let mut start = 0;

    for (i, &byte) in bytes.iter().enumerate() {
        let escape = ESCAPE[byte as usize];
        if escape == 0 {
            continue;
        }

        if start < i {
            writer.write_all(&bytes[start..i])?;
        }

        writer.write_all(&[b'\\', escape])?;

        start = i + 1;
    }

    if start != bytes.len() {
        writer.write_all(&bytes[start..])?;
    }

    writer.write_all(b"\"")?;

    Ok(())
}

// TODO - Radu M
// ensure the escaped sequences are correct

const QU: u8 = b'"'; // \x22
const BS: u8 = b'\\'; // \x5C

// Lookup table of escape sequences. A value of b'x' at index i means that byte
// i is escaped as "\x" in JSON. A value of 0 means that byte i is not escaped.
//
// Adapted from https://github.com/zmanian/canonical_json
// (which was adapted from a repo that no longer exists)
static ESCAPE: [u8; 256] = [
    //  1   2   3   4   5   6   7   8   9   A   B   C   D   E   F
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 1
    0, 0, QU, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 2
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 3
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 4
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, BS, 0, 0, 0, // 5
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 6
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 7
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 8
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 9
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // A
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // B
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // C
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // D
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // E
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // F
];
