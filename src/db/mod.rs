use std::sync::{Arc, RwLock};

#[cfg(not(feature = "redis"))]
pub mod mem;

#[cfg(feature = "redis")]
pub mod redis;

//pub type DbHandle<R, T> = Arc<RwLock<dyn Database<R, E>>>;

pub async fn create_db() -> Arc<RwLock<impl Database>> {
    #[cfg(not(feature = "redis"))]
    let database = mem::InMemoryDb::new();
    #[cfg(feature = "redis")]
    let database = redis::RedisDb::new();
    Arc::new(RwLock::new(database))
}

use std::collections::HashSet;

pub trait Database: Clone {
    type Error: std::fmt::Debug;
    type ETag: std::fmt::Display;

    fn set_value(&mut self, namespace: &str, flag: String) -> Result<bool, Self::Error>;
    fn get_values(&self, namespace: &str) -> Result<HashSet<String>, Self::Error>;
    fn etag(&self, namespace: &str) -> Result<Self::ETag, Self::Error>;
    fn delete_flag(&mut self, namespace: &str, flag: String) -> Result<bool, Self::Error>;
}
