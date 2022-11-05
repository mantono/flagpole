use lazy_static::lazy_static;
use regex::{Regex, RegexBuilder};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

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

/// A String which consists only of characters which are "unreserved" according to RFC3986.
///
/// These are
/// - `a - z`
/// - `A - Z`
/// - `0 - 9`
/// - `-`
/// - `.`
/// - `_`
/// - `~`
///
/// These characters are always safe to use in the path of a URL.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct UnreservedString(String);

impl UnreservedString {
    /// Creates an UnreservedString, if the input accepted.
    ///
    /// ```
    /// UnreservedString::new("Safe-String-123");
    /// ```
    pub fn new<T: Into<String>>(input: T) -> Result<UnreservedString, u8> {
        let input: String = input.into();
        for byte in input.as_bytes() {
            match Self::check(*byte) {
                true => continue,
                false => return Err(*byte),
            }
        }
        Ok(Self(input))
    }

    #[inline]
    const fn check(byte: u8) -> bool {
        match byte {
            b'a'..=b'z' => true,
            b'A'..=b'Z' => true,
            b'0'..=b'9' => true,
            b'-' => true,
            b'.' => true,
            b'_' => true,
            b'~' => true,
            _ => false,
        }
    }

    /// Returns the length of this string. Since the content of this string is only ASCII
    /// characters, the length will be the same in both number of bytes as well as the number of
    /// characters.
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl FromStr for UnreservedString {
    type Err = u8;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl AsRef<str> for UnreservedString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Display for UnreservedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct FlagConf {
    pub rate: f64,
}
