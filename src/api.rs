use std::{
    convert::Infallible,
    sync::{Arc, RwLock},
};

use hyper::{Body, Request, Response};
use routerify::ext::RequestExt;

use crate::{Database, Flag, FlagConf};

type DbHandle = Arc<RwLock<Database>>;

pub async fn head_flags(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::default())
}

pub async fn get_flags(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::default())
}

pub async fn get_flag(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::default())
}

pub async fn put_flag(req: Request<Vec<u8>>) -> Result<Response<Body>, Infallible> {
    let (parts, body) = req.into_parts();
    let body: FlagConf = serde_json::from_slice(&body).unwrap();
    let db: DbHandle = parts.data::<DbHandle>().unwrap().clone();

    let x = db.read().unwrap();
    x.set(flag, rate)
    Ok(Response::default())
}

pub async fn delete_flag(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::default())
}
