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

    let flags = warp::path!("api" / "v1" / "flags" / String);
    let flag = warp::path!("api" / "v1" / "flags" / String / String);
    let head_flags = flags.and(warp::head()).and(db.clone()).map(api::head_flags);
    let get_flags = flags.and(warp::get()).and(db.clone()).map(api::get_flags);
    let put_flag = flag.and(warp::put()).and(db.clone()).map(api::put_flag);
    let delete_flag = flag.and(warp::delete()).and(db.clone()).map(api::delete_flag);

    let routes = put_flag.or(delete_flag).or(head_flags).or(get_flags);

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await
}
