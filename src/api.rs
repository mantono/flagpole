use std::{
    collections::HashMap,
    convert::Infallible,
    slice::SliceIndex,
    sync::{Arc, RwLock},
};

use serde_json::{json, Value};

use crate::flag::Flag;
use crate::flag::FlagConf;
use crate::Database;

type DbHandle = Arc<RwLock<Database>>;

pub async fn head_flags(db: DbHandle) -> () {
    ()
}

pub async fn get_flags(env: String, namespace: String, db: DbHandle) -> HashMap<Flag, FlagConf> {
    db.read()
        .unwrap()
        .data
        .clone()
        .into_iter()
        .filter(|(key, _)| key.namespace() == namespace)
        .collect()
}
/*
pub async fn put_flag(
    flag: Flag,
    conf: FlagConf,
    db: DbHandle,
) -> Result<Response<()>, Infallible> {
    db.write().unwrap().set(flag, conf);
    Ok(Response::default())
}

pub async fn delete_flag(flag: Flag, db: DbHandle) -> impl IntoResponse {
    StatusCode::OK
} */
