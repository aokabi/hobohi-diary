use axum::{
    routing::{get, post},
    Router,
};
use sqlx::MySqlPool;

use crate::handlers::{diary, tag};

pub fn create_router(pool: MySqlPool) -> Router {
    Router::new()
        // エントリ関連のエンドポイント
        .route("/api/entries", get(diary::get_entries))
        .route("/api/entries/with-tags", post(diary::create_entry))
        .route("/api/entries", post(diary::create_simple_entry)) // 後方互換性のため
        .route("/api/entries/count", get(diary::get_entry_count))
        
        // タグ関連のエンドポイント
        .route("/api/tags", get(tag::get_tags).post(tag::create_tag))
        .route("/api/tags/:id/entries", get(tag::get_entries_by_tag))
        
        .with_state(pool)
}
