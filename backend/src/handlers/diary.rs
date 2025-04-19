use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

use crate::models::entry::{CreateEntryRequest, Entry};

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    page: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct EntriesResponse {
    pub entries: Vec<Entry>,
    pub total_pages: u32,
    pub current_page: u32,
}

pub async fn get_entries(
    State(pool): State<MySqlPool>,
    Query(pagination): Query<PaginationQuery>,
) -> impl IntoResponse {
    let page = pagination.page.unwrap_or(1);
    let limit = 10;
    let offset = (page - 1) * limit;
    
    // エントリ取得
    let entries_result = sqlx::query_as::<_, Entry>(
        "SELECT id, content, datetime FROM entry ORDER BY id DESC LIMIT ? OFFSET ?",
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(&pool)
    .await;
    
    // 総数取得
    let count_result = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM entry")
        .fetch_one(&pool)
        .await;
    
    match (entries_result, count_result) {
        (Ok(entries), Ok(count)) => {
            let total_pages = (count as f64 / limit as f64).ceil() as u32;
            let response = EntriesResponse {
                entries,
                total_pages,
                current_page: page,
            };
            (StatusCode::OK, Json(response)).into_response()
        },
        (Err(e), _) | (_, Err(e)) => {
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    }
}

pub async fn create_entry(
    State(pool): State<MySqlPool>,
    Json(request): Json<CreateEntryRequest>,
) -> impl IntoResponse {
    let now = time::OffsetDateTime::now_utc();
    
    match sqlx::query(
        "INSERT INTO entry (content, datetime) VALUES (?, ?)",
    )
    .bind(&request.content)
    .bind(now)
    .execute(&pool)
    .await {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    }
}

pub async fn get_entry_count(
    State(pool): State<MySqlPool>,
) -> impl IntoResponse {
    match sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM entry")
        .fetch_one(&pool)
        .await {
            Ok(count) => (StatusCode::OK, Json(count)).into_response(),
            Err(e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            }
        }
}
