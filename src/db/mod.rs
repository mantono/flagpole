mod mem;
mod redis;

pub type DbHandle = Arc<RwLock<InMemoryDb>>;

pub async fn create_db() -> DbHandle {
    let database = InMemoryDb::new();
    Arc::new(RwLock::new(database))
}
