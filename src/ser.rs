use super::error::{Error, Result};
use itoa;
use serde::ser::Impossible;
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
    type SerializeTuple = Compound<'a, W>;
    type SerializeTupleStruct = Compound<'a, W>;
    type SerializeTupleVariant = Compound<'a, W>;
    type SerializeMap = Compound<'a, W>;
    type SerializeStruct = Compound<'a, W>;
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

    fn serialize_char(self, v: char) -> Result<()> {
        escape_char(&mut self.writer, v)?;
        Ok(())
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        escape_str(&mut self.writer, v)?;
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        use serde::ser::SerializeSeq;
        let mut seq = self.serialize_seq(Some(v.len()))?;
        for byte in v {
            seq.serialize_element(byte)?;
        }
        seq.end()
    }

    fn serialize_none(self) -> Result<()> {
        self.serialize_unit()?;
        Ok(())
    }

    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<()> {
        self.writer.write_all(b"null")?;
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        self.serialize_unit()?;
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        self.serialize_str(variant)?;
        Ok(())
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

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
        self.writer.write_all(b"{")?;
        self.serialize_str(variant)?;
        self.writer.write_all(b":")?;
        value.serialize(&mut *self)?;
        self.writer.write_all(b"}")?;

        Ok(())
    }

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
        self.writer.write_all(b"{")?;
        self.serialize_str(variant)?;
        self.writer.write_all(b":")?;
        self.serialize_seq(Some(len))
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        if len == Some(0) {
            self.writer.write_all(b"{}")?;
            Ok(Compound::Map {
                ser: self,
                state: State::Empty,
            })
        } else {
            self.writer.write_all(b"{")?;
            Ok(Compound::Map {
                ser: self,
                state: State::First,
            })
        }
    }

    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        self.serialize_map(Some(len))
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

impl<'a, W> ser::SerializeTuple for Compound<'a, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<()> {
        ser::SerializeSeq::end(self)
    }
}

impl<'a, W> ser::SerializeTupleStruct for Compound<'a, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    #[inline]
    fn end(self) -> Result<()> {
        ser::SerializeSeq::end(self)
    }
}

impl<'a, W> ser::SerializeTupleVariant for Compound<'a, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    #[inline]
    fn end(self) -> Result<()> {
        match self {
            Compound::Map { ser, state } => {
                match state {
                    State::Empty => {}
                    _ => ser.writer.write_all(b"]")?,
                }

                ser.writer.write_all(b"}")?;
                Ok(())
            }
        }
    }
}

impl<'a, W> ser::SerializeMap for Compound<'a, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<()>
    where
        T: Serialize,
    {
        match *self {
            Compound::Map {
                ref mut ser,
                ref mut state,
            } => {
                // begin object key
                // if the value is not thre first, write a ","
                match *state {
                    State::Rest => ser.writer.write_all(b",")?,
                    _ => (),
                }
                *state = State::Rest;
                key.serialize(&mut **ser)?;

                Ok(())
            }
        }
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        match *self {
            Compound::Map { ref mut ser, .. } => {
                ser.writer.write_all(b":")?;
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
                    _ => ser.writer.write_all(b"}")?,
                }
                Ok(())
            }
        }
    }
}

impl<'a, W> ser::SerializeStruct for Compound<'a, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        match *self {
            Compound::Map { .. } => {
                ser::SerializeMap::serialize_key(self, key)?;
                ser::SerializeMap::serialize_value(self, value)
            }
            #[cfg(feature = "arbitrary_precision")]
            Compound::Number { ref mut ser, .. } => {
                if key == ::number::TOKEN {
                    value.serialize(NumberStrEmitter(&mut *ser))?;
                    Ok(())
                } else {
                    Err(invalid_number())
                }
            }
            #[cfg(feature = "raw_value")]
            Compound::RawValue { ref mut ser, .. } => {
                if key == ::raw::TOKEN {
                    value.serialize(RawValueStrEmitter(&mut *ser))?;
                    Ok(())
                } else {
                    Err(invalid_raw_value())
                }
            }
        }
    }

    #[inline]
    fn end(self) -> Result<()> {
        match self {
            Compound::Map { .. } => ser::SerializeMap::end(self),
            #[cfg(feature = "arbitrary_precision")]
            Compound::Number { .. } => Ok(()),
            #[cfg(feature = "raw_value")]
            Compound::RawValue { .. } => Ok(()),
        }
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
