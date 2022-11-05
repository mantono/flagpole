use std::{collections::HashMap, convert::Infallible};

use crate::flag::{Flag, FlagConf};

pub trait Database {
    type Error;

    fn set_value(&mut self, key: Flag, conf: FlagConf) -> Result<(), Self::Error>;
    fn get_values(&self, namespace: &str) -> Result<HashMap<Flag, FlagConf>, Self::Error>;
    fn delete_flag(&mut self, key: Flag) -> Result<(), Self::Error>;
}

pub struct InMemoryDb {
    data: HashMap<String, HashMap<Flag, FlagConf>>,
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
        let mut data: HashMap<Flag, FlagConf> = self.get_values(key.namespace())?;
        let namespace: String = key.namespace().to_string();
        data.insert(key, conf);
        self.data.insert(namespace, data);
        Ok(())
    }

    fn get_values(&self, namespace: &str) -> Result<HashMap<Flag, FlagConf>, Self::Error> {
        Ok(self.data.get(namespace).map(|data| data.clone()).unwrap_or_default())
    }

    fn delete_flag(&mut self, key: Flag) -> Result<(), Self::Error> {
        self.data.get_mut(key.namespace()).and_then(|v| v.remove(&key));
        Ok(())
    }
}
