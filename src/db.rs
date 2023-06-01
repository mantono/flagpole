use std::{
    collections::{hash_map::DefaultHasher, HashMap, HashSet},
    convert::Infallible,
    hash::{Hash, Hasher},
};

pub trait Database {
    type Error;

    fn set_value(&mut self, namespace: &str, flag: String) -> Result<(), Self::Error>;
    fn get_values(&self, namespace: &str) -> Result<HashSet<String>, Self::Error>;
    fn etag(&self, namespace: &str) -> Result<u64, Self::Error>;
    fn delete_flag(&mut self, namespace: &str, flag: String) -> Result<(), Self::Error>;
}

pub struct InMemoryDb {
    data: HashMap<String, HashSet<String>>,
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
        match self.data.get(namespace) {
            Some(ns_conf) => {
                ns_conf.iter().for_each(|flag| {
                    flag.hash(&mut hasher);
                });
                let hash: u64 = hasher.finish();
                self.etags.insert(namespace.to_owned(), hash);
                hash
            }
            None => 0u64,
        }
    }
}

impl Database for InMemoryDb {
    type Error = Infallible;

    fn set_value(&mut self, namespace: &str, flag: String) -> Result<(), Self::Error> {
        match self.data.get_mut(namespace) {
            Some(flags) => {
                flags.insert(flag);
            }
            None => {
                let mut flags = HashSet::new();
                flags.insert(flag);
                self.data.insert(namespace.to_owned(), flags);
            }
        }
        self.update_etag(namespace);
        Ok(())
    }

    fn get_values(&self, namespace: &str) -> Result<HashSet<String>, Self::Error> {
        let data: HashSet<String> = self.data.get(namespace).map(|m| m.clone()).unwrap_or_default();
        Ok(data)
    }

    fn delete_flag(&mut self, namespace: &str, flag: String) -> Result<(), Self::Error> {
        if let Some(flags) = self.data.get_mut(namespace) {
            flags.remove(&flag);
            self.update_etag(namespace);
        }
        Ok(())
    }

    fn etag(&self, namespace: &str) -> Result<u64, Self::Error> {
        Ok(*self.etags.get(namespace).unwrap_or(&0))
    }
}
