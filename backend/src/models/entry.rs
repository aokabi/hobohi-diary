use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Entry {
    pub id: i32,
    pub content: String,
    pub datetime: OffsetDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateEntryRequest {
    pub content: String,
}
