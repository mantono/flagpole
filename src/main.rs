mod auth;
mod cfg;
mod db;

use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::RwLock;

use axum::response::IntoResponse;
use cfg::Config;
use clap::Parser;
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
    let cfg: Config = Config::parse();
    #[cfg(feature = "logging")]
    env_logger::Builder::new()
        .filter_level(cfg.log_level().to_level_filter())
        .init();

    if cfg.api_key().is_none() {
        log::warn!("No API key is configured, authentication is disabled");
    }

    let state = AppState {
        db: create_db().await,
        api_key: cfg.api_key(),
    };
    let router = Router::new()
        .route("/api/flags/:namespace", get(get_ns).head(head_ns))
        .route("/api/flags/:namespace/:flag", put(put_flag).delete(delete_flag))
        .with_state(state);

    let addr: SocketAddr = cfg.address().unwrap();
    log::info!("Running flagpole on {:?}", addr);
    axum::Server::bind(&addr).serve(router.into_make_service()).await.unwrap();
}

#[derive(Clone)]
struct AppState {
    db: DbHandle,
    api_key: Option<String>,
}

use crate::auth::{accept_auth, ApiKey};
use crate::db::Database;

use axum::extract::{Path, State};
use axum::headers::Authorization;
use axum::Json;
use axum::TypedHeader;
use http::{header, StatusCode};
use std::collections::HashSet;

async fn get_ns(path: Path<String>, state: State<AppState>) -> impl IntoResponse {
    let namespace: String = path.0;
    let db = state.0.db.read().unwrap();
    let etag: &str = db.etag(&namespace).unwrap();
    let flags: HashSet<String> = db.get_values(&namespace).unwrap();
    let resp = Response { namespace, flags };
    (StatusCode::OK, [(header::ETAG, format!("{etag}"))], Json(resp))
}

async fn head_ns(path: Path<String>, state: State<AppState>) -> impl IntoResponse {
    let namespace: String = path.0;
    let db = state.0.db.read().unwrap();
    let etag: &str = db.etag(&namespace).unwrap();
    (StatusCode::OK, [(header::ETAG, format!("{etag}"))])
}

async fn put_flag(
    Path((namespace, flag)): Path<(String, String)>,
    auth: Option<TypedHeader<Authorization<ApiKey>>>,
    state: State<AppState>,
) -> StatusCode {
    if !accept_auth(&state.api_key, auth) {
        return StatusCode::UNAUTHORIZED;
    }
    let updated: bool = state.0.db.write().unwrap().set_value(&namespace, flag.clone()).unwrap();
    if updated {
        #[cfg(feature = "logging")]
        log::info!("Flag {flag} enabled in namespace {namespace}");
    }
    StatusCode::NO_CONTENT
}

async fn delete_flag(
    Path((namespace, flag)): Path<(String, String)>,
    auth: Option<TypedHeader<Authorization<ApiKey>>>,
    state: State<AppState>,
) -> StatusCode {
    if !accept_auth(&state.api_key, auth) {
        return StatusCode::UNAUTHORIZED;
    }
    let updated: bool = state.0.db.write().unwrap().delete_flag(&namespace, flag.clone()).unwrap();
    if updated {
        #[cfg(feature = "logging")]
        log::info!("Flag {flag} disabled in namespace {namespace}");
    }
    StatusCode::NO_CONTENT
}

#[derive(serde::Serialize)]
struct Response {
    pub namespace: String,
    pub flags: HashSet<String>,
}
