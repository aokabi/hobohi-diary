use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

use crate::models::{
    entry::Entry,
    tag::{CreateTagRequest, Tag},
};

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    page: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct TagsResponse {
    pub tags: Vec<Tag>,
}

// タグ一覧を取得
pub async fn get_tags(State(pool): State<MySqlPool>) -> impl IntoResponse {
    match sqlx::query_as::<_, Tag>("SELECT id, name FROM tag ORDER BY name")
        .fetch_all(&pool)
        .await
    {
        Ok(tags) => {
            let response = TagsResponse { tags };
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to fetch tags: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        },
    }
}

// 新しいタグを作成
pub async fn create_tag(
    State(pool): State<MySqlPool>,
    Json(request): Json<CreateTagRequest>,
) -> impl IntoResponse {
    // タグ名が空でないことを確認
    if request.name.trim().is_empty() {
        tracing::warn!("Attempted to create empty tag");
        return (StatusCode::BAD_REQUEST, "Tag name cannot be empty").into_response();
    }

    // 既存のタグを確認
    let existing_tag = sqlx::query_as::<_, Tag>("SELECT id, name FROM tag WHERE name = ?")
        .bind(&request.name)
        .fetch_optional(&pool)
        .await;

    match existing_tag {
        Ok(Some(tag)) => {
            // タグが既に存在する場合は、そのタグを返す
            (StatusCode::OK, Json(tag)).into_response()
        }
        Ok(None) => {
            // 新しいタグを作成
            match sqlx::query("INSERT INTO tag (name) VALUES (?)")
                .bind(&request.name)
                .execute(&pool)
                .await
            {
                Ok(result) => {
                    let id = result.last_insert_id() as i32;
                    let new_tag = Tag {
                        id,
                        name: request.name,
                    };
                    (StatusCode::CREATED, Json(new_tag)).into_response()
                }
                Err(e) => {
                    tracing::error!("Failed to create new tag: {}", e);
                    (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
                }
            }
        }
        Err(e) => {
            tracing::error!("Failed to check existing tag: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    }
}

// タグIDでエントリをフィルタリング
pub async fn get_entries_by_tag(
    State(pool): State<MySqlPool>,
    Path(tag_id): Path<i32>,
    Query(pagination): Query<PaginationQuery>,
) -> impl IntoResponse {
    let page = pagination.page.unwrap_or(1);
    let limit = 10;
    let offset = (page - 1) * limit;

    // タグに関連するエントリを取得
    let entries_result = sqlx::query_as::<_, Entry>(
        r#"
        SELECT e.id, e.content, e.datetime
        FROM entry e
        JOIN entry_tag et ON e.id = et.entry_id
        WHERE et.tag_id = ?
        ORDER BY e.datetime DESC
        LIMIT ? OFFSET ?
        "#,
    )
    .bind(tag_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(&pool)
    .await;

    // 総数取得
    let count_result = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(*)
        FROM entry e
        JOIN entry_tag et ON e.id = et.entry_id
        WHERE et.tag_id = ?
        "#,
    )
    .bind(tag_id)
    .fetch_one(&pool)
    .await;

    match (entries_result, count_result) {
        (Ok(entries), Ok(count)) => {
            let total_pages = (count as f64 / limit as f64).ceil() as u32;
            let response = crate::handlers::diary::EntriesResponse {
                entries,
                total_pages,
                current_page: page,
            };
            (StatusCode::OK, Json(response)).into_response()
        }
        (Err(e), _) | (_, Err(e)) => {
            tracing::error!("Failed to fetch entries by tag: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    }
}
