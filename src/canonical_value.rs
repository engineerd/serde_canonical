use serde_json::value::Value;
use std::fmt::{self, Debug};
use std::{io, str};

pub struct CanonicalValue {
    Value: Value,
}

impl Debug for CanonicalValue {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self.Value {
            Value::Null => formatter.debug_tuple("Null").finish(),
            Value::Bool(v) => formatter.debug_tuple("Bool").field(&v).finish(),
            Value::Number(ref v) => Debug::fmt(v, formatter),
            Value::String(ref v) => formatter.debug_tuple("String").field(v).finish(),
            Value::Array(ref v) => formatter.debug_tuple("Array").field(v).finish(),
            Value::Object(ref v) => formatter.debug_tuple("Object").field(v).finish(),
        }
    }
}


struct WriterFormatter<'a, 'b: 'a> {
    inner: &'a mut fmt::Formatter<'b>,
}

impl<'a, 'b> io::Write for WriterFormatter<'a, 'b> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        fn io_error<E>(_: E) -> io::Error {
            io::Error::new(io::ErrorKind::Other, "fmt error")
        }
        let s = str::from_utf8(buf).map_err(io_error)?;
        self.inner.write_str(s).map_err(io_error)?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl fmt::Display for CanonicalValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut wr = WriterFormatter { inner: f };
        super::ser::to_writer(&mut wr, self).map_err(|_| fmt::Error)
    }
}

impl serde::Serialize for CanonicalValue {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        match self.Value {
            Value::Null => serializer.serialize_unit(),
            Value::Bool(b) => serializer.serialize_bool(b),
            Value::Number(ref n) => n.serialize(serializer),
            Value::String(ref s) => serializer.serialize_str(s),
            Value::Array(ref v) => v.serialize(serializer),
            Value::Object(ref m) => {
                use serde::ser::SerializeMap;
                let mut map = serializer.serialize_map(Some(m.len()))?;
                for (k, v) in m {
                    map.serialize_key(k)?;
                    map.serialize_value(v)?;
                }
                map.end()
            }
        }
    }
}
