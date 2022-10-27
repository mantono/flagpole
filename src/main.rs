mod api;

use std::{
    collections::HashMap,
    convert::Infallible,
    error::Error,
    fmt::Display,
    net::SocketAddr,
    str::FromStr,
    sync::{Arc, Mutex, RwLock},
};

use axum::{
    routing::{get, head},
    Extension, Router,
};
use lazy_static::lazy_static;
use regex::{Regex, RegexBuilder};

#[tokio::main]
async fn main() {
    let router = router();
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    axum::Server::bind(&addr).serve(router.into_make_service()).await.unwrap();
}

fn router() -> Router {
    let db = Arc::new(RwLock::new(Database::new()));

    Router::new()
        .route("/flags", head(api::head_flags).get(api::get_flags))
        .route("/flags/:flag", get(api::get_flag).put(api::put_flag).delete(api::delete_flag))
        .layer(Extension(db))

    /*     Router::builder()
    .data(db)
    .head("/flags", api::head_flags)
    .get("/flags", api::get_flags)
    .put("/flags/:flag", api::put_flag)
    .get("/flags/:flag", api::get_flag)
    .delete("/flags/:flag", api::delete_flag)
    .build()
    .unwrap() */
}

#[derive(serde::Deserialize, Debug)]
pub struct FlagConf {
    pub rate: f64,
}

pub struct Database {
    data: HashMap<Flag, f64>,
}

impl Database {
    pub fn new() -> Database {
        Self {
            data: HashMap::with_capacity(8),
        }
    }
    pub fn get_all(&self) -> HashMap<String, f64> {
        self.data.iter().map(|(k, v)| (k.to_string(), *v)).collect()
    }

    pub fn get(&self, flag: &Flag) -> Option<f64> {
        self.data.get(flag).cloned()
    }

    pub fn set(&mut self, flag: Flag, rate: f64) {
        self.data.insert(flag, rate);
    }

    pub fn delete(&mut self, flag: &Flag) {
        self.data.remove(flag);
    }
}

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct Flag(String);

impl Display for Flag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

const FLAG_KEY_REGEX: &'static str = r"^[a-zA-Z0-9_\.\-]{1,128}$";

lazy_static! {
    static ref REGEX: Regex = RegexBuilder::new(FLAG_KEY_REGEX)
        .size_limit(65_536)
        .build()
        .expect("Compile regex");
}

impl FromStr for Flag {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match REGEX.is_match(s) {
            true => Ok(Flag(s.to_string())),
            false => Err(()),
        }
    }
}

/* fn accept(c: u8) -> bool {
    match c {
        // - OR .
        45 | 46 => true,
        // 0 - 9
        48..=57 => true,
        // A - Z
        65..=90 => true,
        // _
        95 => true,
        97..=122 => true,
        _ => false,
    }
}
 */
