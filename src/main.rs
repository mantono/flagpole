pub mod db;

use std::sync::Arc;
use std::sync::RwLock;

use axum::response::IntoResponse;
use db::InMemoryDb;

type DbHandle = Arc<RwLock<InMemoryDb>>;

async fn create_db() -> DbHandle {
    let database = InMemoryDb::new();
    Arc::new(RwLock::new(database))
}

use axum::routing::{get, put};
use axum::Router;

#[tokio::main]
async fn main() {
    #[cfg(feature = "logging")]
    env_logger::init();
    let db: DbHandle = create_db().await;

    let router = Router::new()
        .route("/api/flags/:namespace", get(get_ns).head(head_ns))
        .route("/api/flags/:namespace/:flag", put(put_flag).delete(delete_flag))
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
use std::collections::HashSet;

async fn get_ns(path: Path<String>, state: State<DbHandle>) -> impl IntoResponse {
    let namespace: String = path.0;
    let db = state.0.read().unwrap();
    let etag: u128 = db.etag(&namespace).unwrap();
    let flags: HashSet<String> = db.get_values(&namespace).unwrap();
    let resp = Response { namespace, flags };
    (StatusCode::OK, [(header::ETAG, format!("{etag}"))], Json(resp))
}

async fn head_ns(path: Path<String>, state: State<DbHandle>) -> impl IntoResponse {
    let namespace: String = path.0;
    let etag: u128 = state.0.read().unwrap().etag(&namespace).unwrap();
    (StatusCode::OK, [(header::ETAG, format!("{etag}"))])
}

async fn put_flag(
    Path((namespace, flag)): Path<(String, String)>,
    state: State<DbHandle>,
) -> StatusCode {
    let updated: bool = state.0.write().unwrap().set_value(&namespace, flag.clone()).unwrap();
    if updated {
        #[cfg(feature = "logging")]
        log::info!("Flag {flag} enabled in namespace {namespace}");
    }
    StatusCode::OK
}

async fn delete_flag(
    Path((namespace, flag)): Path<(String, String)>,
    state: State<DbHandle>,
) -> StatusCode {
    let updated: bool = state.0.write().unwrap().delete_flag(&namespace, flag.clone()).unwrap();
    if updated {
        #[cfg(feature = "logging")]
        log::info!("Flag {flag} disabled in namespace {namespace}");
    }
    StatusCode::OK
}

#[derive(serde::Serialize)]
struct Response {
    pub namespace: String,
    pub flags: HashSet<String>,
}
