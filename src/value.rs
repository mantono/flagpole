use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    Boolean(bool),
    Integer(isize),
    Float(f32),
    String(String),
    Null,
}

impl std::hash::Hash for Value {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match *self {
            Self::Boolean(b) => b.hash(&mut state),
            Self::Integer(i) => i.hash(&mut state),
            Self::String(s) => s.hash(&mut state),
            Self::Float(f) => state.write_u32(f.to_bits()),
            Self::Null => {}
        }
    }
}

impl<'de> serde::Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_enum(?, ?, ?)
    }
}

pub type Config = std::collections::HashMap<String, Value>;
