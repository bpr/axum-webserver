use axum::{
    routing::get,
    routing::put,
    routing::post,
    routing::patch,
    routing::delete,
    Router,
};

use crate::db;
use crate::handlers::{create_entry, read_entry, update_entry, delete_entry, keys};

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
