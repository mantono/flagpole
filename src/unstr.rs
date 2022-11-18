use std::{fmt::Display, hash::Hash, str::FromStr};

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
        matches!(byte, b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~')
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
