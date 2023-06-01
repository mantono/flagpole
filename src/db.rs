use std::{
    collections::{hash_map::DefaultHasher, HashMap, HashSet},
    convert::Infallible,
    hash::{Hash, Hasher},
};

use crate::namespace::Namespace;

pub trait Database {
    type Error;

    fn set_value(&mut self, ns: &Namespace, flag: String) -> Result<(), Self::Error>;
    fn get_values(&self, ns: &Namespace) -> Result<HashSet<String>, Self::Error>;
    fn etag(&self, ns: &Namespace) -> Result<u64, Self::Error>;
    fn delete_flag(&mut self, ns: &Namespace, flag: String) -> Result<(), Self::Error>;
}

pub struct InMemoryDb {
    data: HashMap<Namespace, HashSet<String>>,
    etags: HashMap<Namespace, u64>,
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
    fn update_etag(&mut self, ns: &Namespace) -> u64 {
        let mut hasher = DefaultHasher::new();
        match self.data.get(&ns) {
            Some(ns_conf) => {
                ns_conf.iter().for_each(|flag| {
                    flag.hash(&mut hasher);
                });
                let hash: u64 = hasher.finish();
                self.etags.insert(ns.clone(), hash);
                hash
            }
            None => 0u64,
        }
    }
}

impl Database for InMemoryDb {
    type Error = Infallible;

    fn set_value(&mut self, ns: &Namespace, flag: String) -> Result<(), Self::Error> {
        match self.data.get_mut(&ns) {
            Some(flags) => {
                flags.insert(flag);
            }
            None => {
                let mut flags = HashSet::new();
                flags.insert(flag);
                self.data.insert(ns.clone(), flags);
            }
        }
        self.update_etag(ns);
        Ok(())
    }

    fn get_values(&self, ns: &Namespace) -> Result<HashSet<String>, Self::Error> {
        let data: HashSet<String> = self.data.get(ns).map(|m| m.clone()).unwrap_or_default();
        Ok(data)
    }

    fn delete_flag(&mut self, ns: &Namespace, flag: String) -> Result<(), Self::Error> {
        if let Some(flags) = self.data.get_mut(ns) {
            flags.remove(&flag);
            self.update_etag(ns);
        }
        Ok(())
    }

    fn etag(&self, ns: &Namespace) -> Result<u64, Self::Error> {
        Ok(*self.etags.get(ns).unwrap_or(&0))
    }
}
