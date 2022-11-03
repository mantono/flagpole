use lazy_static::lazy_static;
use regex::{Regex, RegexBuilder};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

use http::Uri;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Flag {
    namespace: Uri,
    key: String,
}

impl Flag {
    pub fn namespace(&self) -> String {
        self.namespace.to_string()
    }

    pub fn key(&self) -> &str {
        &self.key
    }
}

impl Serialize for Flag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.key)
    }
}

impl Display for Flag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}@{}", self.key, self.namespace))
    }
}

const FLAG_KEY_REGEX: &'static str = r"^[a-zA-Z0-9_\.\-]{1,128}$";

lazy_static! {
    static ref REGEX: Regex = RegexBuilder::new(FLAG_KEY_REGEX)
        .size_limit(65_536)
        .build()
        .expect("Compile regex");
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct FlagConf {
    pub rate: f64,
}

/* fn accept(c: u8) -> bool {
    match c {
        // - OR .
        45 | 46 => true,
        // 0 - 9
        48..=57 => true,
        // A - Z
        65..=90 => true,
        // _
        95 => true,
        97..=122 => true,
        _ => false,
    }
}
 */
