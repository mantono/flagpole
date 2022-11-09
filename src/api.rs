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

pub fn get_flags(namespace: String, db: DbHandle) -> warp::reply::Json {
    let values = db.try_read().unwrap().get_values(&namespace).unwrap();
    warp::reply::json(&values)
}

pub fn put_flag(
    namespace: String,
    flag: String,
    conf: FlagConf,
    db: DbHandle,
) -> http::Response<()> {
    let flag = Flag::new(namespace, flag).unwrap();
    db.write().unwrap().set_value(flag, conf).unwrap();
    http::Response::builder().status(http::StatusCode::OK).body(()).unwrap()
}

pub fn delete_flag(namespace: String, flag: String, db: DbHandle) -> http::StatusCode {
    let flag = Flag::new(namespace, flag).unwrap();
    db.write().unwrap().delete_flag(flag).unwrap();
    http::StatusCode::OK
}

struct Resp {
    status: u16,
}

impl warp::Reply for Resp {
    fn into_response(self) -> Response {
        Response::default()
    }
}
