use super::process::LiveCommand;
use super::{Response, Standard};
use alloc::{
    fmt,
    fmt::Display,
    slice::Iter,
    string::{String, ToString},
    vec::{IntoIter, Vec},
};
use core;
use serde::{
    de,
    de::{DeserializeSeed, MapAccess, Visitor},
    forward_to_deserialize_any,
};
#[cfg(test)]
use std::println;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    Generic,
    UnexpectedFields,
    UnexpectedState,
    UnmatchedFields,

    Unknown,
    Msg(String),
}

pub type Result<T> = core::result::Result<T, Error>;

impl de::Error for Error {
    fn custom<T: Display>(_msg: T) -> Self {
        Error::Msg(_msg.to_string())
    }
}
impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(format!("serde deserialize error: {}", self).as_str())
    }
}

#[derive(PartialEq)]
enum DeState {
    WaitForField,
    WaitForValue,
}

pub struct Deserializer<'de> {
    response_iter: &'de Vec<Response>,
    resp_idx: usize,
    struct_field: Option<Iter<'de, &'static str>>,
    cur_resp_field: Option<IntoIter<String>>,
    cur_state: DeState,
}

pub fn from_resp_vec<'a, T>(resp: &'a Vec<Response>) -> Result<T>
where
    T: de::Deserialize<'a>,
{
    let mut deserializer = Deserializer::from(resp);
    let t = T::deserialize(&mut deserializer)?;
    Ok(t)
}

impl<'de> From<&'de LiveCommand> for Deserializer<'de> {
    fn from(s: &'de LiveCommand) -> Self {
        return Self::from(&s.response);
    }
}

impl<'de> From<&'de Vec<Response>> for Deserializer<'de> {
    fn from(s: &'de Vec<Response>) -> Self {
        return Deserializer {
            response_iter: s,
            resp_idx: 0,

            struct_field: None,
            cur_resp_field: None,

            cur_state: DeState::WaitForField,
        };
    }
}

impl<'de> Deserializer<'de> {
    fn next_resp(&mut self) -> Option<&'de Response> {
        if self.resp_idx < self.response_iter.len() {
            let res = Some(&self.response_iter[self.resp_idx]);
            self.resp_idx += 1;
            res
        } else {
            return None;
        }
    }
    fn next_field(&mut self) -> Option<String> {
        let res = match &mut self.struct_field {
            Some(iter) => match iter.next() {
                Some(s) => {
                    self.cur_state = DeState::WaitForValue;
                    Some(String::from(*s))
                }
                _ => {
                    self.struct_field = None;
                    None
                }
            },
            _ => None,
        };

        res
    }
    fn next_value(&mut self) -> Option<String> {
        let res = match &mut self.cur_resp_field {
            Some(iter) => match iter.next() {
                Some(f) => Some(f),
                None => None,
            },
            None => None,
        };

        self.cur_state = DeState::WaitForField;
        res
    }
    fn peek_first_standard(&mut self) -> Option<&'de Standard> {
        while let Some(resp) = self.next_resp() {
            if let Response::Standard(s) = resp {
                self.cur_resp_field = Some(s.parameter.clone().into_iter());
                return Some(s);
            }
        }
        return None;
    }
}

struct SeperatedResponse<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
}

impl<'de, 'a> MapAccess<'de> for SeperatedResponse<'a, 'de> {
    type Error = Error;
    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        match &self.de.struct_field {
            Some(iter) => {
                match iter.size_hint() {
                    (a, Some(_)) => {
                        // do not known why, but work
                        if a == 0 {
                            return Ok(None);
                        } else {
                            return seed.deserialize(&mut *self.de).map(Some);
                        }
                    }
                    _ => Err(Error::Generic),
                }
            }
            _ => Ok(None),
        }
    }
    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        match &self.de.cur_resp_field {
            Some(s) => seed.deserialize(&mut *self.de),
            _ => Err(Error::UnexpectedState),
        }
    }
}

impl<'de, 'a> SeperatedResponse<'a, 'de> {
    pub fn new(de: &'a mut Deserializer<'de>) -> SeperatedResponse<'a, 'de> {
        SeperatedResponse { de: de }
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_string(visitor)
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.peek_first_standard();
        self.struct_field = Some(fields.iter());
        self.deserialize_map(visitor)
    }
    fn deserialize_map<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_map(SeperatedResponse::new(&mut self))
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.cur_state {
            DeState::WaitForField => visitor.visit_string(self.next_field().unwrap()),
            DeState::WaitForValue => match self.next_value() {
                Some(s) => visitor.visit_string(s),
                None => Err(Error::UnmatchedFields),
            },
        }
    }

    forward_to_deserialize_any! {
        str
        option
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
        bytes
        byte_buf
        unit_struct
        newtype_struct
        tuple_struct
        identifier
        tuple
        enum
        ignored_any
        unit
        seq
    }
}

#[cfg(test)]
mod test {
    use super::super::Command;
    use super::from_resp_vec;
    use super::Deserializer;
    use super::Error;
    use alloc::string::String;
    use serde::Deserialize;
    #[derive(Deserialize, Debug, PartialEq)]
    struct foocmd {
        foo1: String,
        foo2: String,
        foo3: String,
    }

    #[test]
    pub fn test_normal() {
        let t = from_resp_vec::<foocmd>(&vec![Command::parse_line(r#"+asd:"hahaha","hohoho",1"#)]);
        assert_eq!(
            t.unwrap(),
            foocmd {
                foo1: String::from("hahaha"),
                foo2: String::from("hohoho"),
                foo3: String::from("1"),
            }
        );
    }
    #[test]
    pub fn test_error_lackfield() {
        assert_eq!(
            from_resp_vec::<foocmd>(&vec![Command::parse_line(r#"+asd:"hahaha","hohoho""#)]),
            Err(Error::UnmatchedFields)
        );
    }
    #[test]

    pub fn test_error_extrafield() {
        assert_eq!(
            from_resp_vec::<foocmd>(&vec![Command::parse_line(r#"+asd:"hahaha","hohoho",1,1"#)]),
            Ok(foocmd{
                foo1: String::from("hahaha"),
                foo2: String::from("hohoho"),
                foo3: String::from("1"),
            })
            // TODO: Should detect extra fields' case
            // Err(Error::UnmatchedFields)
        );
    }
}
