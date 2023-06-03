use std::{
    collections::{HashMap, HashSet},
    convert::Infallible,
};

pub trait Database {
    type Error;

    fn set_value(&mut self, namespace: &str, flag: String) -> Result<(), Self::Error>;
    fn get_values(&self, namespace: &str) -> Result<HashSet<String>, Self::Error>;
    fn etag(&self, namespace: &str) -> Result<u128, Self::Error>;
    fn delete_flag(&mut self, namespace: &str, flag: String) -> Result<(), Self::Error>;
}

pub struct InMemoryDb {
    data: HashMap<String, HashSet<String>>,
    etags: HashMap<String, u128>,
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
    fn update_etag(&mut self, namespace: &str) -> u128 {
        let etag: u128 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
        self.etags.insert(namespace.to_owned(), etag);
        etag
    }
}

impl Database for InMemoryDb {
    type Error = Infallible;

    fn set_value(&mut self, namespace: &str, flag: String) -> Result<(), Self::Error> {
        let updated: bool = match self.data.get_mut(namespace) {
            Some(flags) => flags.insert(flag),
            None => {
                self.data.insert(namespace.to_owned(), HashSet::from_iter(vec![flag]));
                true
            }
        };
        if updated {
            self.update_etag(namespace);
        }
        Ok(())
    }

    fn get_values(&self, namespace: &str) -> Result<HashSet<String>, Self::Error> {
        let data: HashSet<String> = self.data.get(namespace).map(|m| m.clone()).unwrap_or_default();
        Ok(data)
    }

    fn delete_flag(&mut self, namespace: &str, flag: String) -> Result<(), Self::Error> {
        if let Some(flags) = self.data.get_mut(namespace) {
            if flags.remove(&flag) {
                self.update_etag(namespace);
            }
        }
        Ok(())
    }

    fn etag(&self, namespace: &str) -> Result<u128, Self::Error> {
        Ok(*self.etags.get(namespace).unwrap_or(&0))
    }
}
