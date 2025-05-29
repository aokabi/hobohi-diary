use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool; // State抽出のために保持

// サービスとそのエラー型をインポート
use crate::services::tag_service::TagService;
use crate::services::diary_service::DiaryService; // get_entries_by_tagで使用
// リクエスト/レスポンス関連のモデルと構造体を保持
 // get_entries_by_tagのレスポンスで使用
use crate::models::tag::{CreateTagRequest, Tag}; // Tagはレスポンスで使用、CreateTagRequestはcreate_tagで使用
// diaryハンドラーのレスポンス型をインポート (get_entries_by_tagで使用)
use crate::handlers::diary::EntriesResponse;


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
    // TagServiceをインスタンス化
    let tag_service = TagService::new(&pool);

    // サービス層のメソッドを呼び出し
    match tag_service.get_all_tags().await {
        Ok(tags) => {
            let response = TagsResponse { tags };
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to fetch tags: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Error fetching tags: {:?}", e)).into_response()
        }
    }
}

// 新しいタグを作成
pub async fn create_tag(
    State(pool): State<MySqlPool>,
    Json(request): Json<CreateTagRequest>,
) -> impl IntoResponse {
    // ハンドラー層での基本的なバリデーション
    if request.name.trim().is_empty() {
        tracing::warn!("Attempted to create empty tag");
        return (StatusCode::BAD_REQUEST, "Tag name cannot be empty").into_response();
    }

    // TagServiceをインスタンス化
    let tag_service = TagService::new(&pool);

    // サービス層のメソッドを呼び出し
    match tag_service.find_or_create_tag(&request.name).await {
        // find_or_create_tagは常にTagを返す想定（エラーでなければ）
        // 既存ならそのTag、新規作成なら作成されたTag
        // ステータスコードで区別したい場合は、サービス層の戻り値を工夫する必要がある
        // 例: Ok((Tag, bool)) -> boolは新規作成されたかを示すフラグ
        // 今回はシンプルに、成功すればOKかCREATED（区別は難しい）を返す
        Ok(tag) => {
             // 既存か新規作成かの判断は現状できないため、常にCREATEDを返すか、
             // もしくはサービス層の戻り値を変更して判断する。
             // ここではシンプルにCREATEDを返すことにする。
             // より厳密には、サービスがboolフラグを返すように変更するのが望ましい。
            (StatusCode::CREATED, Json(tag)).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to find or create tag: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Error processing tag: {:?}", e)).into_response()
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
    let limit = 10; // 1ページあたりの件数

    // DiaryServiceをインスタンス化
    let diary_service = DiaryService::new(&pool);

    // サービス層のメソッドを呼び出し
    match diary_service.get_entries_by_tag(tag_id, page, limit).await {
        Ok((entries, total_pages)) => {
            // レスポンス構造体を構築
            let response = EntriesResponse { // handlers::diary::EntriesResponse を使用
                entries,
                total_pages,
                current_page: page,
            };
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to fetch entries by tag: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Error fetching entries by tag: {:?}", e)).into_response()
        }
    }
}
