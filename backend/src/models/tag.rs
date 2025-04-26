use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Tag {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTagRequest {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TagResponse {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EntryTag {
    pub entry_id: i32,
    pub tag_id: i32,
}

// エントリ作成時にタグを指定するためのリクエスト拡張
#[derive(Debug, Deserialize)]
pub struct EntryWithTags {
    pub content: String,
    pub tags: Vec<String>, // タグ名のリスト
}
