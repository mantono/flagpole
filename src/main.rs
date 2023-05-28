mod api;
mod db;
mod flag;
mod unstr;

use std::sync::{Arc, RwLock};

use db::InMemoryDb;

type DbHandle = Arc<RwLock<InMemoryDb>>;

async fn create_db() -> DbHandle {
    let database = InMemoryDb::new();
    Arc::new(RwLock::new(database))
}

use axum::routing::{delete, get, head, patch, put};
use axum::Router;

#[tokio::main]
async fn main() {
    let Databasease: DbHandle = create_db().await;

    let router = Router::new()
        .route("/api/flags/:namespace", get(get_ns).head(head_ns).put(put_ns).patch(patch_ns))
        .route("/api/flags/:namespace/:flag", delete(delete_flag));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}

use axum::extract::Path;

async fn get_ns(namespace: Path<String>) {}
async fn head_ns(namespace: Path<String>) {}
async fn put_ns(namespace: Path<String>) {}
async fn patch_ns(namespace: Path<String>) {}
async fn delete_flag(namespace: Path<String>) {}
