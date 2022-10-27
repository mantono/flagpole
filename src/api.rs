use std::{
    collections::HashMap,
    convert::Infallible,
    slice::SliceIndex,
    sync::{Arc, RwLock},
};

use axum::{
    extract::Path,
    http::{Request, Response, StatusCode},
    response::IntoResponse,
    Extension, Json,
};
use serde_json::{json, Value};

use crate::{Database, Flag, FlagConf};

type DbHandle = Arc<RwLock<Database>>;

pub async fn head_flags(Extension(db): Extension<DbHandle>) -> () {
    ()
}

pub async fn get_flags(Extension(db): Extension<DbHandle>) -> Json<HashMap<String, f64>> {
    Json(db.read().unwrap().get_all())
}

pub async fn get_flag(Path(flag): Path<String>, Extension(db): Extension<DbHandle>) -> Json<Value> {
    let flag: Flag = flag.parse().unwrap();
    let rate: f64 = db.read().unwrap().get(&flag).unwrap_or_default();
    Json(json!({ "rate": 0 }))
}

pub async fn put_flag(
    req: Request<FlagConf>,
    Path(flag): Path<String>,
    Extension(db): Extension<DbHandle>,
) -> Result<Response<()>, Infallible> {
    let body: &FlagConf = req.body();
    let flag: Flag = flag.parse().unwrap();
    db.write().unwrap().set(flag, body.rate);
    Ok(Response::default())
}

pub async fn delete_flag(Path(flag): Path<String>) -> impl IntoResponse {
    let flag: Flag = flag.parse().unwrap();
    StatusCode::OK
}
