use std::collections::{HashMap, HashSet};

use redis::{Connection, RedisError};

use super::Database;

#[derive(Clone)]
pub struct RedisDb {
    client: redis::Client,
    etags: HashMap<String, String>,
}

impl RedisDb {
    pub fn new(uri: String) -> Self {
        Self {
            client: redis::Client::open(uri).unwrap(),
            etags: HashMap::with_capacity(4),
        }
    }

    fn get_conn(&self) -> Result<Connection, RedisError> {
        self.client.get_connection()
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

impl Database for RedisDb {
    type Error = RedisError;
    type ETag = String;

    fn set_value(&mut self, namespace: &str, flag: String) -> Result<bool, Self::Error> {
        let mut connection = self.get_conn()?;
        let added: u8 =
            redis::cmd("SADD").arg(&[namespace, flag.as_str()]).query(&mut connection)?;
        let updated: bool = added != 0u8;

        if updated {
            self.update_etag(namespace);
        }

        Ok(updated)
    }

    fn get_values(&self, namespace: &str) -> Result<HashSet<String>, Self::Error> {
        let mut connection = self.get_conn()?;
        let members: HashSet<String> =
            redis::cmd("SMEMBERS").arg(&[namespace]).query(&mut connection)?;
        Ok(members)
    }

    fn etag(&self, namespace: &str) -> Result<Self::ETag, Self::Error> {
        let etag = match self.etags.get(namespace) {
            Some(etag) => etag.to_string(),
            None => String::default(),
        };
        Ok(etag)
    }

    fn delete_flag(&mut self, namespace: &str, flag: String) -> Result<bool, Self::Error> {
        let mut connection = self.get_conn()?;
        let deleted: u8 =
            redis::cmd("SREM").arg(&[namespace, flag.as_str()]).query(&mut connection)?;
        let updated = deleted != 0u8;

        if updated {
            self.update_etag(namespace);
        }

        Ok(updated)
    }
}
