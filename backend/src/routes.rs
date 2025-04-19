use axum::{
    routing::{get, post},
    Router,
};
use sqlx::MySqlPool;

use crate::handlers::diary;

pub fn create_router(pool: MySqlPool) -> Router {
    Router::new()
        .route("/api/entries", get(diary::get_entries).post(diary::create_entry))
        .route("/api/entries/count", get(diary::get_entry_count))
        .with_state(pool)
}
