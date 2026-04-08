mod auth;
mod cfg;
mod db;

use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::RwLock;

use axum::response::IntoResponse;
use cfg::Config;
use clap::Parser;

use axum::Router;
use axum::routing::{get, put};
use db::Database;

#[tokio::main]
async fn main() {
    let cfg: Config = Config::parse();
    #[cfg(feature = "logging")]
    init_logs(&cfg);

    if cfg.api_key().is_none() {
        log::warn!("No API key is configured, authentication is disabled");
    }

    let state = AppState {
        db: db::create_db(&cfg).await,
        api_key: cfg.api_key(),
    };
    let router = Router::new()
        .route("/api/health", get(health_check))
        .route("/api/flags/:namespace", get(get_ns).head(head_ns))
        .route("/api/flags/:namespace/:flag", put(put_flag).delete(delete_flag))
        .with_state(state);

    let addr: SocketAddr = cfg.address().unwrap();
    log::info!("Running flagpole on {:?}", addr);

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    log::info!("Server shutdown complete");
}

#[cfg(feature = "logging")]
#[cfg(not(feature = "json-logging"))]
fn init_logs(cfg: &Config) {
    env_logger::Builder::new()
        .filter_level(cfg.log_level().to_level_filter())
        .init();
}

#[cfg(feature = "json-logging")]
fn init_logs(cfg: &Config) {
    env_logger::Builder::new()
        .target(env_logger::Target::Stdout)
        .filter_level(cfg.log_level().to_level_filter())
        .format(|buf, record| {
            use std::io::Write;
            let log_entry = serde_json::json!({
                "timestamp": chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
                "level": record.level().to_string(),
                "target": record.target(),
                "message": record.args().to_string(),
            });
            writeln!(buf, "{}", log_entry)
        })
        .init();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c().await.expect("failed to install CTRL+C signal handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            log::info!("Received SIGINT (CTRL+C), initiating graceful shutdown");
        },
        _ = terminate => {
            log::info!("Received SIGTERM, initiating graceful shutdown");
        },
    }
}

#[derive(Clone)]
struct AppState<T>
where
    T: Database,
{
    db: Arc<RwLock<T>>,
    api_key: Option<String>,
}

use crate::auth::{ApiKey, accept_auth};

use axum::Json;
use axum::TypedHeader;
use axum::extract::{Path, State};
use axum::headers::Authorization;
use http::{StatusCode, header};
use std::collections::HashSet;

async fn get_ns(
    path: Path<String>,
    state: State<AppState<impl Database>>,
) -> Result<impl IntoResponse, StatusCode> {
    let namespace: String = path.0;
    let db = state.0.db.read().map_err(|_| {
        log::error!("Database lock poisoned");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    let etag: String = db.etag(&namespace);
    let flags: HashSet<String> = db.get_values(&namespace).unwrap();
    let resp = Response { namespace, flags };
    Ok((StatusCode::OK, [(header::ETAG, etag)], Json(resp)))
}

async fn head_ns(
    path: Path<String>,
    state: State<AppState<impl Database>>,
) -> Result<impl IntoResponse, StatusCode> {
    let namespace: String = path.0;
    let db = state.0.db.read().map_err(|_| {
        log::error!("Database lock poisoned");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    let etag: String = db.etag(&namespace);
    Ok((StatusCode::OK, [(header::ETAG, etag)]))
}

const MAX_NAME_LENGTH: usize = 256;

async fn put_flag(
    Path((namespace, flag)): Path<(String, String)>,
    auth: Option<TypedHeader<Authorization<ApiKey>>>,
    state: State<AppState<impl Database>>,
) -> StatusCode {
    if !accept_auth(&state.api_key, auth) {
        return StatusCode::UNAUTHORIZED;
    }
    if namespace.len() > MAX_NAME_LENGTH || flag.len() > MAX_NAME_LENGTH {
        return StatusCode::BAD_REQUEST;
    }
    let mut db = match state.0.db.write() {
        Ok(db) => db,
        Err(_) => {
            log::error!("Database lock poisoned");
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    };
    let updated: bool = db.set_value(&namespace, flag.clone()).unwrap();
    if updated {
        log::info!("Flag '{flag}' enabled in namespace <<{namespace}>>");
    }
    StatusCode::NO_CONTENT
}

async fn delete_flag(
    Path((namespace, flag)): Path<(String, String)>,
    auth: Option<TypedHeader<Authorization<ApiKey>>>,
    state: State<AppState<impl Database>>,
) -> StatusCode {
    if !accept_auth(&state.api_key, auth) {
        return StatusCode::UNAUTHORIZED;
    }
    if namespace.len() > MAX_NAME_LENGTH || flag.len() > MAX_NAME_LENGTH {
        return StatusCode::BAD_REQUEST;
    }
    let mut db = match state.0.db.write() {
        Ok(db) => db,
        Err(_) => {
            log::error!("Database lock poisoned");
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    };
    let updated: bool = db.delete_flag(&namespace, flag.clone()).unwrap();
    if updated {
        log::info!("Flag {flag} disabled in namespace {namespace}");
    }
    StatusCode::NO_CONTENT
}

async fn health_check(state: State<AppState<impl Database>>) -> StatusCode {
    let db = match state.0.db.read() {
        Ok(db) => db,
        Err(_) => {
            log::error!("Database lock poisoned");
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    };
    match db.health_check() {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::SERVICE_UNAVAILABLE,
    }
}

#[derive(serde::Serialize)]
struct Response {
    pub namespace: String,
    pub flags: HashSet<String>,
}
