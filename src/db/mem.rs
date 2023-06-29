use std::{
    collections::{HashMap, HashSet},
    convert::Infallible,
};

pub struct InMemoryDb {
    data: HashMap<String, HashSet<String>>,
    etags: HashMap<String, String>,
}

impl InMemoryDb {
    pub fn new() -> Self {
        Self {
            data: HashMap::with_capacity(4),
            etags: HashMap::with_capacity(4),
        }
    }

    fn update_etag(&mut self, namespace: &str) -> u128 {
        let etag: u128 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
        self.etags.insert(namespace.to_owned(), format!("{etag:x}"));
        etag
    }
}

impl Default for InMemoryDb {
    fn default() -> Self {
        Self::new()
    }
}

impl crate::db::Database for InMemoryDb {
    type Error = Infallible;
    type ETag = String;

    fn set_value(&mut self, namespace: &str, flag: String) -> Result<bool, Self::Error> {
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
        Ok(updated)
    }

    fn get_values(&self, namespace: &str) -> Result<HashSet<String>, Self::Error> {
        let data: HashSet<String> = self.data.get(namespace).cloned().unwrap_or_default();
        Ok(data)
    }

    fn delete_flag(&mut self, namespace: &str, flag: String) -> Result<bool, Self::Error> {
        let updated: bool = match self.data.get_mut(namespace) {
            Some(flags) => flags.remove(&flag),
            None => false,
        };
        if updated {
            self.update_etag(namespace);
        }
        Ok(updated)
    }

    fn etag(&self, namespace: &str) -> Result<String, Self::Error> {
        let etag = match self.etags.get(namespace) {
            Some(etag) => etag.to_string(),
            None => String::default(),
        };
        Ok(etag)
    }
}
