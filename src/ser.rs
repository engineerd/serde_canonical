use super::error::{Error, Result};
use itoa;
use serde::ser::Impossible;
use serde::{ser, Serialize};
use std::{i64, io, num::FpCategory};

pub fn to_writer<W, T>(writer: W, value: &T) -> Result<()>
where
    T: serde::Serialize,
    T: ?Sized,
    W: io::Write,
{
    let mut ser = Serializer::new(writer);
    super::canonical_value::to_value(value)?.serialize(&mut ser)?;
    Ok(())
}

pub fn to_vec<T>(value: &T) -> Result<Vec<u8>>
where
    T: serde::Serialize,
    T: ?Sized,
{
    let mut writer = Vec::with_capacity(128);
    to_writer(&mut writer, value)?;
    Ok(writer)
}

pub fn to_string<T>(value: &T) -> Result<String>
where
    T: Serialize,
    T: ?Sized,
{
    Ok(unsafe { String::from_utf8_unchecked(to_vec(value)?) })
}

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

impl<'a, W> ser::Serializer for &'a mut Serializer<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    type SerializeSeq = OrderedKeyCompound<'a, W>;
    type SerializeTuple = OrderedKeyCompound<'a, W>;
    type SerializeTupleStruct = OrderedKeyCompound<'a, W>;
    type SerializeTupleVariant = OrderedKeyCompound<'a, W>;
    type SerializeMap = OrderedKeyCompound<'a, W>;
    type SerializeStruct = OrderedKeyCompound<'a, W>;
    type SerializeStructVariant = OrderedKeyCompound<'a, W>;

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
            Ok(OrderedKeyCompound::Map {
                ser: self,
                state: State::Empty,
                cur_key: None,
            })
        } else {
            self.writer.write_all(b"[")?;
            Ok(OrderedKeyCompound::Map {
                ser: self,
                state: State::First,
                cur_key: None,
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
            Ok(OrderedKeyCompound::Map {
                ser: self,
                state: State::Empty,
                cur_key: None,
            })
        } else {
            self.writer.write_all(b"{")?;
            Ok(OrderedKeyCompound::Map {
                ser: self,
                state: State::First,
                cur_key: None,
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
        len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        self.writer.write_all(b"{")?;
        self.serialize_str(variant)?;
        self.writer.write_all(b":")?;
        self.serialize_map(Some(len))
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


impl<'a, W> ser::SerializeSeq for OrderedKeyCompound<'a, W>
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
            OrderedKeyCompound::Map {
                ref mut ser,
                ref mut state,
                ..
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
            OrderedKeyCompound::Map { ser, state, .. } => {
                match state {
                    State::Empty => {}
                    _ => ser.writer.write_all(b"]")?,
                }

                Ok(())
            }
        }
    }
}

impl<'a, W> ser::SerializeTuple for OrderedKeyCompound<'a, W>
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

impl<'a, W> ser::SerializeTupleStruct for OrderedKeyCompound<'a, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<()> {
        ser::SerializeSeq::end(self)
    }
}

impl<'a, W> ser::SerializeTupleVariant for OrderedKeyCompound<'a, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<()> {
        match self {
            OrderedKeyCompound::Map { ser, state, .. } => {
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

#[doc(hidden)]
pub enum OrderedKeyCompound<'a, W: 'a>
where
    W: io::Write,
{
    Map {
        ser: &'a mut Serializer<W>,
        cur_key: Option<String>,
        state: State,
    },
}

impl<'a, W> ser::SerializeMap for OrderedKeyCompound<'a, W>
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
            OrderedKeyCompound::Map {
                ref mut ser,
                ref mut cur_key,
                ref mut state,
            } => {
                // begin object key
                // if the value is not thre first, write a ","
                match *state {
                    State::Rest => ser.writer.write_all(b",")?,
                    _ => (),
                }
                *state = State::Rest;

                let mut key_serializer = AscendingKeySerializer {
                    ser: ser,
                    cur_key: cur_key.take(),
                };

                key.serialize(&mut key_serializer)?;

                Ok(())
            }
        }
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        match *self {
            OrderedKeyCompound::Map { ref mut ser, .. } => {
                ser.writer.write_all(b":")?;
                value.serialize(&mut **ser)?;

                Ok(())
            }
        }
    }

    fn end(self) -> Result<()> {
        match self {
            OrderedKeyCompound::Map { ser, state, .. } => {
                match state {
                    State::Empty => {}
                    _ => ser.writer.write_all(b"}")?,
                }
                Ok(())
            }
        }
    }
}


impl<'a, W> ser::SerializeStruct for OrderedKeyCompound<'a, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        match *self {
            OrderedKeyCompound::Map { .. } => {
                ser::SerializeMap::serialize_key(self, key)?;
                ser::SerializeMap::serialize_value(self, value)
            }

        }
    }

    fn end(self) -> Result<()> {
        match self {
            OrderedKeyCompound::Map { .. } => ser::SerializeMap::end(self),
        }
    }
}


impl<'a, W> ser::SerializeStructVariant for OrderedKeyCompound<'a, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        match self {
            OrderedKeyCompound::Map { .. } => {
                ser::SerializeStruct::serialize_field(self, key, value)
            }
        }
    }

    fn end(self) -> Result<()> {
        match self {
            OrderedKeyCompound::Map { ser, state, .. } => {
                match state {
                    State::Empty => {}
                    _ => ser.writer.write_all(b"}")?,
                }

                ser.writer.write_all(b"}")?;
                Ok(())
            }
        }
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


struct AscendingKeySerializer<'a, W>
where
    W: io::Write,
{
    ser: &'a mut Serializer<W>,
    cur_key: Option<String>,
}

impl<'a, W> ser::Serializer for &'a mut AscendingKeySerializer<'a, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Impossible<(), Error>;
    type SerializeTuple = Impossible<(), Error>;
    type SerializeTupleStruct = Impossible<(), Error>;
    type SerializeTupleVariant = Impossible<(), Error>;
    type SerializeMap = Impossible<(), Error>;
    type SerializeStruct = Impossible<(), Error>;
    type SerializeStructVariant = Impossible<(), Error>;

    fn serialize_str(self, v: &str) -> Result<()> {
        match self.cur_key {
            Some(ref cur_key) if v == cur_key => {
                Err(Error::Custom(String::from(format!("repeated key: {}", v))))
            }
            Some(ref cur_key) if v < cur_key => {
                Err(Error::Custom(String::from(format!("unordered key: {}", v))))
            }
            _ => {
                self.cur_key = Some(v.to_string());
                self.ser.serialize_str(v)
            }
        }
    }

    fn serialize_bool(self, v: bool) -> Result<()> {
        Err(Error::Custom(String::from(format!(
            "key must be a string: {}",
            v
        ))))
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        Err(Error::Custom(String::from(format!(
            "key must be a string: {}",
            v
        ))))
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        Err(Error::Custom(String::from(format!(
            "key must be a string: {}",
            v
        ))))
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        Err(Error::Custom(String::from(format!(
            "key must be a string: {}",
            v
        ))))
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        Err(Error::Custom(String::from(format!(
            "key must be a string: {}",
            v
        ))))
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        Err(Error::Custom(String::from(format!(
            "key must be a string: {}",
            v
        ))))
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        Err(Error::Custom(String::from(format!(
            "key must be a string: {}",
            v
        ))))
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        Err(Error::Custom(String::from(format!(
            "key must be a string: {}",
            v
        ))))
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        Err(Error::Custom(String::from(format!(
            "key must be a string: {}",
            v
        ))))
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        Err(Error::Custom(String::from(format!(
            "key must be a string: {}",
            v
        ))))
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        Err(Error::Custom(String::from(format!(
            "key must be a string: {}",
            v
        ))))
    }

    fn serialize_char(self, v: char) -> Result<()> {
        Err(Error::Custom(String::from(format!(
            "key must be a string: {}",
            v
        ))))
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<()> {
        Err(Error::Custom(String::from(
            format!("key must be a string",),
        )))
    }

    fn serialize_none(self) -> Result<()> {
        Err(Error::Custom(String::from(
            format!("key must be a string",),
        )))
    }

    fn serialize_some<T>(self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::Custom(String::from(
            format!("key must be a string",),
        )))
    }

    fn serialize_unit(self) -> Result<()> {
        Err(Error::Custom(String::from(
            format!("key must be a string",),
        )))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        Err(Error::Custom(String::from(
            format!("key must be a string",),
        )))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<()> {
        Err(Error::Custom(String::from(
            format!("key must be a string",),
        )))
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::Custom(String::from(
            format!("key must be a string",),
        )))
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::Custom(String::from(
            format!("key must be a string",),
        )))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Err(Error::Custom(String::from(
            format!("key must be a string",),
        )))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Err(Error::Custom(String::from(
            format!("key must be a string",),
        )))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Err(Error::Custom(String::from(
            format!("key must be a string",),
        )))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Err(Error::Custom(String::from(
            format!("key must be a string",),
        )))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Err(Error::Custom(String::from(
            format!("key must be a string",),
        )))
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Err(Error::Custom(String::from(
            format!("key must be a string",),
        )))
    }

    // Struct variants are represented in JSON as `{ NAME: { K: V, ... } }`.
    // This is the externally tagged representation.
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(Error::Custom(String::from(
            format!("key must be a string",),
        )))
    }
}