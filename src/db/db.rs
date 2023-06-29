use std::{
    collections::{HashMap, HashSet},
    convert::Infallible,
};

pub trait Database {
    type Error;
    type ETag: std::fmt::Display;

    fn set_value(&mut self, namespace: &str, flag: String) -> Result<bool, Self::Error>;
    fn get_values(&self, namespace: &str) -> Result<HashSet<String>, Self::Error>;
    fn etag(&self, namespace: &str) -> Result<Self::ETag, Self::Error>;
    fn delete_flag(&mut self, namespace: &str, flag: String) -> Result<bool, Self::Error>;
}
