use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    convert::Infallible,
    hash::{Hash, Hasher},
};

use crate::flag::{Flag, FlagConf};

pub trait Database {
    type Error;

    fn set_value(&mut self, key: Flag, conf: FlagConf) -> Result<(), Self::Error>;
    fn get_values(&self, namespace: &str) -> Result<HashMap<Flag, FlagConf>, Self::Error>;
    fn etag(&self, namespace: &str) -> Result<u64, Self::Error>;
    fn delete_flag(&mut self, key: Flag) -> Result<(), Self::Error>;
}

pub struct InMemoryDb {
    data: HashMap<Flag, FlagConf>,
    etags: HashMap<String, u64>,
}

impl InMemoryDb {
    pub fn new() -> Self {
        Self {
            data: HashMap::with_capacity(4),
            etags: HashMap::with_capacity(4),
        }
    }
}

impl InMemoryDb {
    fn update_etag(&mut self, namespace: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.data
            .iter()
            .filter(|(k, _)| k.namespace() == namespace)
            .map(|(k, v)| (k.clone(), v.clone()))
            .for_each(|(k, v)| {
                k.hash(&mut hasher);
                v.hash(&mut hasher);
            });
        let hash: u64 = hasher.finish();
        self.etags.insert(namespace.to_string(), hash);
        hash
    }
}

impl Database for InMemoryDb {
    type Error = Infallible;

    fn set_value(&mut self, key: Flag, conf: FlagConf) -> Result<(), Self::Error> {
        let namespace: String = key.namespace().to_string();
        self.data.insert(key, conf);
        self.update_etag(&namespace);
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
        self.update_etag(key.namespace());
        Ok(())
    }

    fn etag(&self, namespace: &str) -> Result<u64, Self::Error> {
        Ok(*self.etags.get(namespace).unwrap_or(&0))
    }
}
