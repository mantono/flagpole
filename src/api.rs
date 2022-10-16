use std::{
    collections::HashMap,
    convert::Infallible,
    hash::Hash,
    sync::{Arc, RwLock},
};

use actix_web::{
    delete,
    dev::Response,
    get, head, put,
    web::{self, Data, Path},
    HttpRequest, HttpResponse, Responder, Result,
};

use crate::{Database, Flag, FlagConf};

type DbHandle = Data<RwLock<Database>>;

#[get("/flags/{flag}")]
pub async fn get_flag(req: HttpRequest, db: DbHandle) -> Result<impl Responder> {
    let flag: Flag = req.match_info().get("flag").unwrap().parse().unwrap();
    let rate: f64 = db.read().unwrap().get(&flag).unwrap_or_default();
    Ok(web::Json(serde_json::json!({ "rate": rate })))
}

#[put("/flags/{flag}")]
pub async fn put_flag(req: HttpRequest, db: DbHandle) -> impl Responder {
    HttpResponse::Ok()
}

#[delete("/flags/{flag}")]
pub async fn delete_flag(req: HttpRequest, db: DbHandle) -> impl Responder {
    HttpResponse::Ok()
}

#[head("/flags")]
pub async fn head_flags(req: HttpRequest, db: DbHandle) -> impl Responder {
    HttpResponse::Ok()
}

#[get("/flags")]
pub async fn get_flags(req: HttpRequest, db: DbHandle) -> impl Responder {
    let flags: HashMap<String, f64> = db.read().unwrap().get_all();
    let json: String = serde_json::to_string(&flags).unwrap();
    HttpResponse::Ok().body(json)
}
