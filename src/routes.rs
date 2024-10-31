use axum::{routing::delete, routing::get, routing::patch, routing::post, routing::put, Router};

use crate::db;
use crate::handlers::{create_entry, delete_entry, keys, read_entry, update_entry};

pub fn new() -> Router<db::Db> {
    Router::new()
        .route("/create", post(create_entry))
        .route("/read/:key", get(read_entry))
        .route("/keys", get(keys))
        .route("/update/:key", put(update_entry))
        .route("/update/:key", patch(update_entry))
        .route("/delete/:key", delete(delete_entry))
    //        .with_state(db)
}
