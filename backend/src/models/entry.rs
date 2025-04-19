use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Entry {
    pub id: i32,
    pub content: String,
    pub datetime: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateEntryRequest {
    pub content: String,
}
