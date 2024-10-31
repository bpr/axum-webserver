use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub(crate) type Db = Arc<RwLock<HashMap<String, String>>>;
// pub(crate) struct Db(Arc<RwLock<HashMap<String, String>>>);

pub fn new() -> Db {
    Arc::new(RwLock::new(HashMap::new()))
}

pub async fn insert(db: Db, key: String, value: String) -> Option<String> {
    let mut hash_map = db.write().await;
    hash_map.insert(key, value)
}

pub async fn read(db: Db, key: String) -> Option<String> {
    let hash_map = db.read().await;
    hash_map.get(&key).cloned()
}

pub async fn delete(db: Db, key: String) -> Option<String> {
    let mut hash_map = db.write().await;
    hash_map.remove(&key)
}

pub async fn get_keys(db: Db) -> Vec<String> {
    let hash_map = db.read().await;
    hash_map.keys().cloned().collect()
}
