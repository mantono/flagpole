use crate::{
    db::{self, Database},
    flag::{Flag, FlagConf},
    DbHandle,
};

pub fn head_flags(namespace: String, db: DbHandle) -> String {
    format!("HEAD flags for namespace {}", namespace)
}

pub fn get_flags(namespace: String, db: DbHandle) -> String {
    format!(
        "GET flags for namespace {}: {:?}",
        &namespace,
        db.try_read().unwrap().get_values(&namespace)
    )
}

pub fn put_flag(namespace: String, flag: String, db: DbHandle) -> String {
    let msg: String = format!("PUT flag {} for namespace {}", flag, namespace);
    let flag = Flag::new(namespace, flag).unwrap();
    let conf = FlagConf { rate: 1.0 };
    db.write().unwrap().set_value(flag, conf).unwrap();
    msg
}

pub fn delete_flag(namespace: String, flag: String, db: DbHandle) -> String {
    format!("DELETE flag {} for namespace {}", flag, namespace)
}
