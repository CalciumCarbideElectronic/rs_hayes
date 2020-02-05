use super::process::LiveCommand;
use super::Response;
use alloc::boxed::Box;
use alloc::fmt;
use alloc::fmt::Display;
use core;
use serde::de;
use serde::de::Visitor;
use serde::forward_to_deserialize_any;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    Generic,
}

pub type Result<T> = core::result::Result<T, Error>;

impl de::Error for Error {
    fn custom<T: Display>(_msg: T) -> Self {
        Error::Generic
    }
}
impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("De Error")
    }
}

pub struct Deserializer<'de> {
    response_iter: Box<dyn Iterator<Item = Response>>,
    _phantom: &'de core::marker::PhantomData<u8>,
}

// pub fn from_str<'a, T>(s: &'a str) -> Result<T>
// where
//     T: Deserializer<'a>,
// {
//     let mut deserializer = Deserializer::from_str(s);
//     let t = T::deserialize(&mut deserializer)?;
//     if deserializer.input_raw.is_empty() {
//         Ok(t)
//     } else {
//         Err(Error::TrailingCharacters)
//     }
// }
impl<'de> Deserializer<'de> {
    fn next_resp(&mut self) -> Option<Response> {
        self.response_iter.next()
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::Generic)
    }

    forward_to_deserialize_any! {
        bool
        u8
        u16
        u32
        u64
        i8
        i16
        i32
        i64
        f32
        f64
        char
        str
        string
        option
        bytes
        byte_buf
        unit_struct
        newtype_struct
        tuple_struct
        struct
        identifier
        tuple
        enum
        ignored_any
        unit
        seq
        map
    }
}

#[cfg(test)]
mod test {
    use super::super::Command;
    #[test]
    pub fn test_foo() {
        Command::parse_line(r#"+asd:"foo1","foo2",foo3"#);
    }
}
