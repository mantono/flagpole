use std::{
    collections::HashMap,
    convert::Infallible,
    sync::{Arc, RwLock},
};

use http::Response;
use serde_json::json;
use warp::{reply::Json, Filter};

use crate::{Database, Flag, FlagConf};

type DbHandle = Arc<RwLock<Database>>;

pub fn routes(
    db: DbHandle,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let get_flag = get_flag(db.clone());
    let get_all_flags = warp::get().and(warp::path("flags")).map(|| Ok("foo"));
    let head_flags = warp::head().and(warp::path("flags")).map(|| "foo");

    let put_flag = warp::put()
        .and(warp::path!("flags" / Flag))
        .and(warp::body::json())
        .map(|p: Flag, b: FlagConf| format!("P: {}, B: {:?} ", p, b));

    let all = get_flag.or(get_all_flags).or(head_flags).or(put_flag);
    all
}

fn with_db(db: DbHandle) -> impl Filter<Extract = (DbHandle,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}

fn get_flag(
    db: DbHandle,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path!("flags" / Flag))
        .and(with_db(db))
        .map(|flag: Flag, db: DbHandle| db.read().unwrap().get(&flag).unwrap_or_default())
        .map(|rate: f64| {
            let json = json!({ "rate": rate });
            warp::reply::json(&json)
        })
}

fn get_all_flags(
    db: DbHandle,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get().and(warp::path("flags")).map(|| Ok("foo"))
}
