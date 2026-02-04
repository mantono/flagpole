use crate::cfg::Config;
use std::collections::HashSet;
use std::sync::{Arc, RwLock};

#[cfg(not(feature = "redis"))]
pub mod mem;

#[cfg(feature = "redis")]
pub mod redis;

pub async fn create_db(cfg: &Config) -> Arc<RwLock<impl Database + use<>>> {
    #[cfg(not(feature = "redis"))]
    #[cfg(feature = "logging")]
    log::info!("Using InMemoryDb");
    #[cfg(not(feature = "redis"))]
    let database = mem::InMemoryDb::new();

    #[cfg(feature = "redis")]
    #[cfg(feature = "logging")]
    log::info!("Using RedisDb");
    #[cfg(feature = "redis")]
    let database = redis::RedisDb::new(cfg.redis_uri().to_string());

    Arc::new(RwLock::new(database))
}

pub trait Database: Clone {
    type Error: std::fmt::Debug;

    fn set_value(&mut self, namespace: &str, flag: String) -> Result<bool, Self::Error>;
    fn get_values(&self, namespace: &str) -> Result<HashSet<String>, Self::Error>;
    fn etag(&self, namespace: &str) -> String;
    fn delete_flag(&mut self, namespace: &str, flag: String) -> Result<bool, Self::Error>;
    fn health_check(&self) -> Result<(), Self::Error>;
}
