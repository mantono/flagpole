mod db;
mod namespace;
mod unstr;
mod value;

use crate::value::Config;
use std::sync::{Arc, RwLock};

use axum::response::IntoResponse;
use db::InMemoryDb;

type DbHandle = Arc<RwLock<InMemoryDb>>;

async fn create_db() -> DbHandle {
    let database = InMemoryDb::new();
    Arc::new(RwLock::new(database))
}

use axum::routing::{delete, get, head, patch, put};
use axum::Router;
use namespace::Namespace;

#[tokio::main]
async fn main() {
    let db: DbHandle = create_db().await;

    let router = Router::new()
        .route("/api/flags/:namespace", get(get_ns).head(head_ns).put(put_ns).patch(patch_ns))
        .route("/api/flags/:namespace/:flag", delete(delete_flag))
        .with_state(db);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}

use crate::db::Database;
use axum::extract::{Path, State};
use axum::Json;
use http::{header, StatusCode};

async fn get_ns(namespace: Path<String>, state: State<DbHandle>) -> (StatusCode, String) {
    (StatusCode::OK, String::from("Hello world"))
}

async fn head_ns(path: Path<String>, state: State<DbHandle>) -> impl IntoResponse {
    let ns: Namespace = path.0.parse().unwrap();
    let etag: u64 = state.0.read().unwrap().etag(&ns).unwrap();

    (StatusCode::OK, [(header::ETAG, format!("{etag}"))])
}

async fn put_ns(
    namespace: Path<String>,
    body: Json<Config>,
    state: State<DbHandle>,
) -> (StatusCode, String) {
    (StatusCode::OK, format!("{:?}", body.0))
}

async fn patch_ns(namespace: Path<String>, state: State<DbHandle>) -> (StatusCode, String) {
    (StatusCode::OK, String::from("Hello world"))
}

async fn delete_flag(namespace: Path<String>, state: State<DbHandle>) -> (StatusCode, String) {
    (StatusCode::OK, String::from("Hello world"))
}
