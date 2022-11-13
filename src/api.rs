use http::StatusCode;
use warp::{hyper::Body, reply::Response};

use crate::{
    db::Database,
    flag::{Flag, FlagConf},
    DbHandle,
};

pub fn head_flags(namespace: String, db: DbHandle) -> http::StatusCode {
    http::StatusCode::OK
}

pub fn get_flags(namespace: String, db: DbHandle) -> http::Response<String> {
    let values = db.try_read().unwrap().get_values(&namespace).unwrap();
    //warp::reply::json(&values)
    let json: String = serde_json::to_string(&values).unwrap();
    http::Response::builder()
        .status(http::StatusCode::OK)
        .header("ETag", "")
        .body(json)
        .unwrap()
}

pub fn put_flag(
    namespace: String,
    flag: String,
    conf: FlagConf,
    db: DbHandle,
) -> http::Response<String> {
    let flag = Flag::new(namespace, flag).unwrap();
    db.write().unwrap().set_value(flag, conf).unwrap();
    http::Response::builder()
        .status(http::StatusCode::OK)
        .body(String::from(""))
        .unwrap()
}

pub fn delete_flag(namespace: String, flag: String, db: DbHandle) -> http::StatusCode {
    let flag = Flag::new(namespace, flag).unwrap();
    db.write().unwrap().delete_flag(flag).unwrap();
    http::StatusCode::OK
}
