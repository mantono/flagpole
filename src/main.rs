mod api;
mod flag;

use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use flag::{Flag, FlagConf};
use http::request;

fn main() {
    rouille::start_server("0.0.0.0:8080", move |request| rouille::Response::text("foo"))
}

/* Router::new()
.route("/flags/:namespace", head(api::head_flags).get(api::get_flags))
.route("/flags/:namespace/:flag", put(api::put_flag).delete(api::delete_flag))
.layer(Extension(db)) */

/*     Router::builder()
.data(db)
.head("/flags", api::head_flags)
.get("/flags", api::get_flags)
.put("/flags/:flag", api::put_flag)
.get("/flags/:flag", api::get_flag)
.delete("/flags/:flag", api::delete_flag)
.build()
.unwrap() */

pub struct Database {
    data: HashMap<Flag, FlagConf>,
}

impl Database {
    pub fn new() -> Database {
        Self {
            data: HashMap::with_capacity(8),
        }
    }
    pub fn get_all(&self) -> HashMap<String, FlagConf> {
        self.data.iter().map(|(k, v)| (k.to_string(), v.clone())).collect()
    }

    pub fn get(&self, flag: &Flag) -> Option<FlagConf> {
        self.data.get(flag).cloned()
    }

    pub fn set(&mut self, flag: Flag, conf: FlagConf) {
        self.data.insert(flag, conf);
    }

    pub fn delete(&mut self, flag: &Flag) {
        self.data.remove(flag);
    }
}
