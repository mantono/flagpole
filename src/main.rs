mod api;
mod db;
mod flag;

use std::{
    collections::HashMap,
    convert::Infallible,
    sync::{Arc, RwLock},
};

use db::{Database, InMemoryDb};
use flag::{Flag, FlagConf};
use http::request;
use warp::Filter;

type DbHandle = Arc<RwLock<InMemoryDb>>;

async fn create_db() -> DbHandle {
    let database = InMemoryDb::new();
    Arc::new(RwLock::new(database))
}

#[tokio::main]
async fn main() {
    let database: DbHandle = create_db().await;
    let db = warp::any().map(move || database.clone());
    let base = warp::path!("api" / "v1" / "flags" / String);
    let head_flags = base
        .and(warp::head())
        .and(db.clone())
        .map(|namespace, db| format!("HEAD flags for namespace {}", namespace));

    let get_flags = base.and(warp::get()).and(db.clone()).map(|namespace: String, db: DbHandle| {
        format!(
            "GET flags for namespace {}: {:?}",
            &namespace,
            db.try_read().unwrap().get_values(&namespace)
        )
    });

    let put_flag = base
        .and(warp::path!(String))
        .and(warp::put())
        .and(db.clone())
        .map(|namespace, flag, db| format!("PUT flag {} for namespace {}", flag, namespace));

    let delete_flag = base
        .and(warp::path!(String))
        .and(warp::delete())
        .and(db.clone())
        .map(|namespace, flag, db| format!("DELETE flag {} for namespace {}", flag, namespace));

    let routes = head_flags.or(get_flags).or(put_flag).or(delete_flag);

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await
}
