use std::{
    collections::HashMap,
    fmt::Display,
    str::FromStr,
    sync::{Arc, Mutex},
};

use lazy_static::lazy_static;
use regex::{Regex, RegexBuilder};
use warp::Filter;

#[tokio::main]
async fn main() {
    let db = Arc::new(Mutex::new(Database::new()));
    let get_flag = warp::get()
        .and(warp::path!("flags" / FlagKey))
        .map(move |flag: FlagKey| format!("Got flag {:?}", db.lock().unwrap().get(&flag)));

    let get_all_flags = warp::get()
        .and(warp::path("flags"))
        .map(|| String::from("Get ALL flags"));

    let head_flags = warp::head().and(warp::path("flags")).map(|| "foo");

    let put_flag = warp::put()
        .and(warp::path!("flags" / String))
        .and(warp::body::json())
        .map(|p: String, b: Flag| format!("P: {}, B: {:?} ", p, b));

    let all = get_flag.or(get_all_flags).or(head_flags).or(put_flag);
    warp::serve(all).run(([127, 0, 0, 1], 8080)).await;
}

#[derive(serde::Deserialize, Debug)]
pub struct Flag {
    pub rate: f64,
}

struct Database {
    data: HashMap<FlagKey, f64>,
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

    pub fn get(&self, flag: &FlagKey) -> Option<f64> {
        self.data.get(flag).cloned()
    }

    pub fn set(&mut self, flag: FlagKey, rate: f64) {
        self.data.insert(flag, rate);
    }

    pub fn delete(&mut self, flag: &FlagKey) {
        self.data.remove(flag);
    }
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct FlagKey(String);

impl Display for FlagKey {
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

impl FromStr for FlagKey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match REGEX.is_match(s) {
            true => Ok(FlagKey(s.to_string())),
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
