use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub(crate) type Db = Arc<RwLock<HashMap<String, String>>>;
// pub(crate) struct Db(Arc<RwLock<HashMap<String, String>>>);

    pub fn new() -> Db {
        Arc::new(RwLock::new(HashMap::new()))
    }

    pub fn insert(db: Db, key: String, value: String) -> Option<String> {
        let mut hash_map = db.write().expect("RwLock poisoned");
        hash_map.insert(key, value)
    }

    pub fn read(db: Db, key: String) -> Option<String> {
        let hash_map = db.read().expect("RwLock poisoned");
        hash_map.get(&key).cloned()
    }

    pub fn delete(db: Db, key: String) -> Option<String> {
        let mut hash_map = db.write().expect("RwLock poisoned");
        hash_map.remove(&key)
    }

    pub fn get_keys(db: Db) -> Vec<String> {
        let hash_map = db.read().expect("RwLock poisoned");
        hash_map.keys().cloned().collect()
    }

