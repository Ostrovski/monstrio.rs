use self::chrono::{DateTime, UTC};
use self::serde::ser::{Serialize, Serializer};

extern crate chrono;
extern crate serde;

#[derive(Debug, PartialEq)]
pub enum Value<'t> {
    Int(i64),
    UInt(u64),
    Float(f64),
    DateTime(DateTime<UTC>),
    Str(&'t str),
}

impl<'t> Serialize for Value<'t> {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        match *self {
            Value::Int(v) => serializer.serialize_i64(v),
            Value::UInt(v) => serializer.serialize_u64(v),
            Value::Float(v) => serializer.serialize_f64(v),
            Value::Str(v) => serializer.serialize_str(v),
            Value::DateTime(ref v) => serializer.serialize_str(&format!("{}", v)),
        }
    }
}
