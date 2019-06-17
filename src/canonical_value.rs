use super::error::Error;
use serde::Serialize;
use serde_json::value::*;
use std::io::Cursor;

pub struct CanonicalValue {
    value: Value,
}

impl serde::Serialize for CanonicalValue {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        match self.value {
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


pub fn to_value<T>(value: T) -> Result<CanonicalValue, Error>
where
    T: Serialize,
{
    ensure_canonical(&value)?;
    let val = serde_json::to_value(value)?;
    Ok(CanonicalValue { value: val })
}

fn ensure_canonical<T>(value: &T) -> Result<(), Error>
where
    T: serde::Serialize,
    T: ?Sized,
{
    let c = Cursor::new(Vec::new());
    let mut ser = super::ser::Serializer::new(c);
    value.serialize(&mut ser)?;
    Ok(())
}
