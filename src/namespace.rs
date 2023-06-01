use std::fmt::Display;

use crate::unstr::UnreservedString;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Namespace(UnreservedString);

impl std::str::FromStr for Namespace {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let unres: UnreservedString = s.parse().map_err(|_| Error::Character)?;
        match unres.len() {
            MIN_LEN..=MAX_LEN => Ok(Namespace(unres)),
            _ => Err(Error::Length),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Error {
    Length,
    Character,
}

const MIN_LEN: usize = 1;
const MAX_LEN: usize = 128;

impl Display for Namespace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.as_ref())
    }
}
