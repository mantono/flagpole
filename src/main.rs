mod api;

use std::{
    collections::HashMap,
    fmt::Display,
    str::FromStr,
    sync::{Arc, Mutex, RwLock},
};

use actix_web::{web, App, HttpServer};
use lazy_static::lazy_static;
use regex::{Regex, RegexBuilder};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let db = web::Data::new(RwLock::new(Database::new()));

    HttpServer::new(move || {
        App::new()
            .service(api::get_flags)
            .service(api::head_flags)
            .service(api::get_flag)
            .service(api::put_flag)
            .service(api::delete_flag)
            .app_data(db.clone())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
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
