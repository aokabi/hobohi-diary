use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool; // State抽出のために保持

// サービスとそのエラー型をインポート
use crate::services::diary_service::DiaryService;
// リクエスト/レスポンス関連のモデルと構造体を保持
use crate::models::entry::{CreateEntryRequest, Entry}; // Entryは当面保持、EntriesResponseが削除されれば後で削除検討
use crate::models::tag::{EntryWithTags, Tag}; // TagはEntryWithTagsResponseで使用、EntryWithTagsはcreate_entryで使用

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

#[derive(Debug, Serialize)]
pub struct EntryWithTagsResponse {
    pub id: i32,
    pub content: String,
    pub datetime: chrono::NaiveDateTime,
    pub tags: Vec<Tag>,
}

#[derive(Debug, Serialize)]
pub struct EntriesWithTagsResponse {
    pub entries: Vec<EntryWithTagsResponse>,
    pub total_pages: u32,
    pub current_page: u32,
}

pub async fn get_entries(
    State(pool): State<MySqlPool>,
    Query(pagination): Query<PaginationQuery>,
) -> impl IntoResponse {
    let page = pagination.page.unwrap_or(1);
    let limit = 10; // 1ページあたりの件数

    // DiaryServiceをインスタンス化
    let diary_service = DiaryService::new(&pool);

    // サービス層のメソッドを呼び出し
    match diary_service.get_entries_with_details(page, limit).await {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(e) => {
            // DiaryServiceErrorを適切なHTTPレスポンスに変換
            tracing::error!("Failed to fetch entries: {:?}", e);
            // エラーの種類に応じてステータスコードを変えることも可能
            // 例: match e { DiaryServiceError::NotFound => ..., _ => ... }
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Error fetching entries: {:?}", e)).into_response()
        }
    }
}

pub async fn create_entry(
    State(pool): State<MySqlPool>,
    Json(request): Json<EntryWithTags>,
) -> impl IntoResponse {
    // DiaryServiceをインスタンス化
    let diary_service = DiaryService::new(&pool);

    // サービス層のメソッドを呼び出し
    match diary_service.create_entry_with_tags(&request).await {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(e) => {
            tracing::error!("Failed to create entry with tags: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Error creating entry: {:?}", e)).into_response()
        }
    }
}

// 従来のCreateEntryRequestを使用するエンドポイントも残しておく（後方互換性のため）
pub async fn create_simple_entry(
    State(pool): State<MySqlPool>,
    Json(request): Json<CreateEntryRequest>,
) -> impl IntoResponse {
    // DiaryServiceをインスタンス化
    let diary_service = DiaryService::new(&pool);

    // サービス層のメソッドを呼び出し
    match diary_service.create_simple_entry(&request).await {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(e) => {
            tracing::error!("Failed to create simple entry: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Error creating simple entry: {:?}", e)).into_response()
        }
    }
}

pub async fn get_entry_count(State(pool): State<MySqlPool>) -> impl IntoResponse {
    // DiaryServiceをインスタンス化
    let diary_service = DiaryService::new(&pool);

    // サービス層のメソッドを呼び出し
    match diary_service.get_entry_count().await {
        Ok(count) => (StatusCode::OK, Json(count)).into_response(),
        Err(e) => {
            tracing::error!("Failed to get entry count: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Error getting entry count: {:?}", e)).into_response()
        }
    }
}
