use std::sync::{Arc, RwLock};

use self::mem::InMemoryDb;

pub mod db;

#[cfg(not(feature = "redis"))]
pub mod mem;

#[cfg(feature = "redis")]
pub mod redis;

#[cfg(not(feature = "redis"))]
pub type DbHandle = Arc<RwLock<InMemoryDb>>;

#[cfg(feature = "redis")]
pub type DbHandle = Arc<RwLock<Redis>>;

pub async fn create_db() -> DbHandle {
    #[cfg(feature = "redis")]
    let database = redis::Redis::new();

    #[cfg(not(feature = "redis"))]
    let database = InMemoryDb::new();

    Arc::new(RwLock::new(database))
}
