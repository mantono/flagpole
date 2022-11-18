use crate::{
    db::Database,
    flag::{Flag, FlagConf},
    DbHandle,
};

pub fn head_flags(namespace: String, db: DbHandle) -> http::Response<String> {
    let etag: u64 = db.read().unwrap().etag(&namespace).unwrap();
    http::Response::builder()
        .header("etag", etag)
        .status(http::StatusCode::OK)
        .body(String::from(""))
        .unwrap()
}

pub fn get_flags(namespace: String, db: DbHandle) -> http::Response<String> {
    let dbx = db.try_read().unwrap();
    let values = dbx.get_values(&namespace).unwrap();
    let json: String = serde_json::to_string(&values).unwrap();
    let etag: u64 = dbx.etag(&namespace).unwrap();
    http::Response::builder()
        .status(http::StatusCode::OK)
        .header("ETag", etag)
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
