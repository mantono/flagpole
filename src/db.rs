use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    convert::Infallible,
    hash::Hasher,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::flag::{Flag, FlagConf};

pub trait Database {
    type Error;

    fn set_value(&mut self, key: Flag, conf: FlagConf) -> Result<(), Self::Error>;
    fn get_values(&self, namespace: &str) -> Result<HashMap<Flag, FlagConf>, Self::Error>;
    fn etag(&self, namespace: &str) -> Result<String, Self::Error>;
    fn delete_flag(&mut self, key: Flag) -> Result<(), Self::Error>;
}

pub struct InMemoryDb {
    data: HashMap<Flag, FlagConf>,
}

impl InMemoryDb {
    pub fn new() -> Self {
        Self {
            data: HashMap::with_capacity(4),
        }
    }
}

impl Database for InMemoryDb {
    type Error = Infallible;

    fn set_value(&mut self, key: Flag, conf: FlagConf) -> Result<(), Self::Error> {
        self.data.insert(key, conf);
        Ok(())
    }

    fn get_values(&self, namespace: &str) -> Result<HashMap<Flag, FlagConf>, Self::Error> {
        let data: HashMap<Flag, FlagConf> = self
            .data
            .iter()
            .filter(|(k, _)| k.namespace() == namespace)
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        Ok(data)
    }

    fn delete_flag(&mut self, key: Flag) -> Result<(), Self::Error> {
        self.data.remove(&key);
        Ok(())
    }

    fn etag(&self, namespace: &str) -> Result<String, Self::Error> {
        Ok(todo!())
    }
}
