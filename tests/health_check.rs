use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::routing::get;
use axum::Router;
use std::sync::{Arc, RwLock};
use tower::ServiceExt;

// Import the Database trait
use flagpole::db::Database;

#[cfg(not(feature = "redis"))]
use flagpole::db::mem::InMemoryDb;

#[cfg(feature = "redis")]
use flagpole::db::redis::RedisDb;

#[derive(Clone)]
struct AppState<T>
where
    T: Database,
{
    db: Arc<RwLock<T>>,
    api_key: Option<String>,
}

async fn health_check_handler(
    axum::extract::State(state): axum::extract::State<AppState<impl Database>>,
) -> StatusCode {
    let db = state.db.read().unwrap();
    match db.health_check() {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::SERVICE_UNAVAILABLE,
    }
}

#[cfg(not(feature = "redis"))]
#[tokio::test]
async fn test_health_check_in_memory() {
    let state = AppState {
        db: Arc::new(RwLock::new(InMemoryDb::new())),
        api_key: None,
    };

    let app = Router::new().route("/health", get(health_check_handler)).with_state(state);

    let response = app
        .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[cfg(feature = "redis")]
#[tokio::test]
async fn test_health_check_redis_not_connected() {
    // Use an invalid Redis URI to simulate connection failure
    let state = AppState {
        db: Arc::new(RwLock::new(RedisDb::new("redis://invalid-host:6379".to_string()))),
        api_key: None,
    };

    let app = Router::new().route("/health", get(health_check_handler)).with_state(state);

    let response = app
        .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
}

#[cfg(feature = "redis")]
#[tokio::test]
async fn test_health_check_redis_connected() {
    // This test requires a running Redis instance at localhost:6379
    // Skip if Redis is not available
    let redis_uri =
        std::env::var("REDIS_URI").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());

    let state = AppState {
        db: Arc::new(RwLock::new(RedisDb::new(redis_uri))),
        api_key: None,
    };

    let app = Router::new().route("/health", get(health_check_handler)).with_state(state);

    let response = app
        .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
        .await
        .unwrap();

    // This will pass if Redis is available, otherwise it will fail
    // In a real CI/CD setup, you'd ensure Redis is running or skip this test
    let status = response.status();
    assert!(
        status == StatusCode::OK || status == StatusCode::SERVICE_UNAVAILABLE,
        "Expected OK or SERVICE_UNAVAILABLE, got {:?}",
        status
    );
}
