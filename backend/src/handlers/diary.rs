use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

use crate::models::{
    entry::{CreateEntryRequest, Entry},
    tag::{EntryWithTags, Tag},
};

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
            
            // エントリごとにタグを取得
            let mut entries_with_tags = Vec::new();
            for entry in entries {
                let tags_result = sqlx::query_as::<_, Tag>(
                    r#"
                    SELECT t.id, t.name
                    FROM tag t
                    JOIN entry_tag et ON t.id = et.tag_id
                    WHERE et.entry_id = ?
                    ORDER BY t.name
                    "#,
                )
                .bind(entry.id)
                .fetch_all(&pool)
                .await;

                match tags_result {
                    Ok(tags) => {
                        entries_with_tags.push(EntryWithTagsResponse {
                            id: entry.id,
                            content: entry.content,
                            datetime: entry.datetime,
                            tags,
                        });
                    }
                    Err(_) => {
                        // タグ取得に失敗した場合は空のタグリストで続行
                        entries_with_tags.push(EntryWithTagsResponse {
                            id: entry.id,
                            content: entry.content,
                            datetime: entry.datetime,
                            tags: Vec::new(),
                        });
                    }
                }
            }

            let response = EntriesWithTagsResponse {
                entries: entries_with_tags,
                total_pages,
                current_page: page,
            };
            (StatusCode::OK, Json(response)).into_response()
        }
        (Err(e), _) | (_, Err(e)) => {
            tracing::error!("Failed to fetch entries: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    }
}

pub async fn create_entry(
    State(pool): State<MySqlPool>,
    Json(request): Json<EntryWithTags>,
) -> impl IntoResponse {
    let now = chrono::Utc::now();

    // トランザクション開始
    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            tracing::error!("Failed to begin transaction: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
        }
    };

    // エントリの作成
    let entry_result = sqlx::query("INSERT INTO entry (content, datetime) VALUES (?, ?)")
        .bind(&request.content)
        .bind(now)
        .execute(&mut *tx)
        .await;

    let entry_id = match entry_result {
        Ok(result) => result.last_insert_id() as i32,
        Err(e) => {
            tracing::error!("Failed to create entry: {}", e);
            let _ = tx.rollback().await;
            return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
        }
    };

    // タグの処理
    for tag_name in &request.tags {
        if tag_name.trim().is_empty() {
            continue; // 空のタグはスキップ
        }

        // タグが存在するか確認し、なければ作成
        let tag_id = match sqlx::query_scalar::<_, i32>("SELECT id FROM tag WHERE name = ?")
            .bind(tag_name)
            .fetch_optional(&mut *tx)
            .await
        {
            Ok(Some(id)) => id, // タグが存在する場合
            Ok(None) => {
                // タグが存在しない場合は作成
                match sqlx::query("INSERT INTO tag (name) VALUES (?)")
                    .bind(tag_name)
                    .execute(&mut *tx)
                    .await
                {
                    Ok(result) => result.last_insert_id() as i32,
                    Err(e) => {
                        tracing::error!("Failed to create tag: {}", e);
                        let _ = tx.rollback().await;
                        return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
                    }
                }
            }
            Err(e) => {
                tracing::error!("Failed to fetch tag: {}", e);
                let _ = tx.rollback().await;
                return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
            }
        };

        // エントリとタグの関連付け
        if let Err(e) = sqlx::query("INSERT INTO entry_tag (entry_id, tag_id) VALUES (?, ?)")
            .bind(entry_id)
            .bind(tag_id)
            .execute(&mut *tx)
            .await
        {
            tracing::error!("Failed to associate entry with tag: {}", e);
            let _ = tx.rollback().await;
            return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
        }
    }

    // トランザクションのコミット
    match tx.commit().await {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(e) => {
            tracing::error!("Failed to fetch entries: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        },
    }
}

// 従来のCreateEntryRequestを使用するエンドポイントも残しておく（後方互換性のため）
pub async fn create_simple_entry(
    State(pool): State<MySqlPool>,
    Json(request): Json<CreateEntryRequest>,
) -> impl IntoResponse {
    let now = chrono::Utc::now();

    match sqlx::query("INSERT INTO entry (content, datetime) VALUES (?, ?)")
        .bind(&request.content)
        .bind(now)
        .execute(&pool)
        .await
    {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(e) => {
            tracing::error!("Failed to create simple entry: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    }
}

pub async fn get_entry_count(State(pool): State<MySqlPool>) -> impl IntoResponse {
    match sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM entry")
        .fetch_one(&pool)
        .await
    {
        Ok(count) => (StatusCode::OK, Json(count)).into_response(),
        Err(e) => {
            tracing::error!("Failed to get entry count: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    }
}
