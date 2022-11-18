use serde::Serialize;
use std::{fmt::Display, hash::Hash};

use crate::unstr::UnreservedString;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Flag {
    namespace: UnreservedString,
    key: UnreservedString,
}

const MIN_LEN: usize = 1;
const MAX_LEN: usize = 128;

impl Flag {
    pub fn new<N: Into<String>, K: Into<String>>(namespace: N, key: K) -> Result<Flag, FlagError> {
        let namespace: UnreservedString = Self::parse(namespace, FlagField::Namespace)?;
        let key: UnreservedString = Self::parse(key, FlagField::Key)?;
        Ok(Self { namespace, key })
    }

    fn parse<T: Into<String>>(input: T, field: FlagField) -> Result<UnreservedString, FlagError> {
        UnreservedString::new(input.into())
            .map_err(|byte: u8| FlagError::InvalidByte(field, byte))
            .and_then(|string: UnreservedString| match string.len() {
                MIN_LEN..=MAX_LEN => Ok(string),
                0 => Err(FlagError::Length(field, 0)),
                len => Err(FlagError::Length(field, len)),
            })
    }

    pub fn namespace(&self) -> &str {
        self.namespace.as_ref()
    }

    pub fn key(&self) -> &str {
        self.key.as_ref()
    }
}

#[derive(Debug, Clone)]
pub enum FlagError {
    Length(FlagField, usize),
    InvalidByte(FlagField, u8),
}

impl Display for FlagError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg: String = match self {
            FlagError::Length(field, len) => {
                format!(
                    "{:?} had an invalid length: {}. Expected length {} <= {}",
                    field, len, MIN_LEN, MAX_LEN
                )
            }
            FlagError::InvalidByte(field, byte) => {
                format!("{:?} contained an invalid byte: {}", field, byte)
            }
        };
        f.write_str(&msg)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum FlagField {
    Namespace,
    Key,
}

impl Serialize for Flag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.key.as_ref())
    }
}

impl Display for Flag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}@{}", self.key, self.namespace))
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct FlagConf {
    pub rate: f64,
}

impl Hash for FlagConf {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write(&self.rate.to_be_bytes());
    }
}
