use crate::db::{delete, get_keys, insert, read, Db};
use axum::{
    debug_handler,
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};

// the input to our `create_user` handler
#[derive(Deserialize, Serialize)]
pub(crate) struct Entry {
    key: String,
    value: String,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Value {
    value: String,
}

#[debug_handler]
pub async fn create_entry(
    State(db): State<Db>,
    Json(payload): Json<Entry>,
) -> (StatusCode, Json<Entry>) {
    insert(db, payload.key.clone(), payload.value.clone()).await;
    (StatusCode::OK, Json(payload))
}

#[debug_handler]
pub async fn read_entry(
    State(db): State<Db>,
    Path(key): Path<String>,
) -> (StatusCode, Json<String>) {
    match read(db, key.clone()).await {
        Some(value) => (StatusCode::OK, Json(value)),
        None => (StatusCode::NOT_FOUND, Json(key)),
    }
}

#[debug_handler]
pub async fn update_entry(
    State(db): State<Db>,
    Path(key): Path<String>,
    Json(value): Json<Value>,
) -> (StatusCode, Json<Value>) {
    match insert(db, key.clone(), value.value.clone()).await {
        Some(s) => (StatusCode::OK, Json(Value { value: s })),
        None => (StatusCode::NOT_FOUND, Json(value)),
    }
}

#[debug_handler]
pub async fn delete_entry(
    State(db): State<Db>,
    Path(key): Path<String>,
) -> (StatusCode, Json<String>) {
    match delete(db, key.clone()).await {
        Some(_) => (StatusCode::OK, Json(key)),
        None => (StatusCode::NOT_FOUND, Json(key)),
    }
}

#[debug_handler]
pub async fn keys(State(db): State<Db>) -> (StatusCode, Json<Vec<String>>) {
    let v = get_keys(db).await;
    match v.as_slice() {
        [] => (StatusCode::NOT_FOUND, Json(v)),
        _ => (StatusCode::OK, Json(v)),
    }
}
