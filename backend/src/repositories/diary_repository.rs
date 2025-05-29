use sqlx::{MySqlPool, Result};
use crate::models::entry::Entry;
 // create_entry_with_tags で使うため

pub struct DiaryRepository<'a> {
    pool: &'a MySqlPool,
}

impl<'a> DiaryRepository<'a> {
    pub fn new(pool: &'a MySqlPool) -> Self {
        Self { pool }
    }

    /// 指定されたページのエントリを取得します。
    pub async fn find_entries(&self, limit: u32, offset: u32) -> Result<Vec<Entry>> {
        sqlx::query_as::<_, Entry>(
            "SELECT id, content, datetime FROM entry ORDER BY id DESC LIMIT ? OFFSET ?",
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(self.pool)
        .await
    }

    /// エントリの総数を取得します。
    pub async fn count_entries(&self) -> Result<i64> {
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM entry")
            .fetch_one(self.pool)
            .await
    }

    /// 新しいエントリを作成します（タグなし）。
    pub async fn create_simple_entry(&self, content: &str) -> Result<u64> {
        let now = chrono::Utc::now();
        let result = sqlx::query("INSERT INTO entry (content, datetime) VALUES (?, ?)")
            .bind(content)
            .bind(now)
            .execute(self.pool)
            .await?;
        Ok(result.last_insert_id())
    }

    /// 新しいエントリと関連するタグを作成します（トランザクション内）。
    /// 戻り値は作成されたエントリのIDです。
    pub async fn create_entry_with_tags_tx(
        tx: &mut sqlx::Transaction<'_, sqlx::MySql>,
        content: &str,
    ) -> Result<i32> {
        let now = chrono::Utc::now();
        let result = sqlx::query("INSERT INTO entry (content, datetime) VALUES (?, ?)")
            .bind(content)
            .bind(now)
            .execute(&mut **tx) // Dereference tx to get &mut MySqlConnection
            .await?;
        Ok(result.last_insert_id() as i32)
    }

    /// 指定されたタグIDに関連付けられたエントリを取得します。
    pub async fn find_entries_by_tag(
        &self,
        tag_id: i32,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<Entry>> {
        sqlx::query_as::<_, Entry>(
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
        .fetch_all(self.pool)
        .await
    }

    /// 指定されたタグIDに関連付けられたエントリの総数を取得します。
    pub async fn count_entries_by_tag(&self, tag_id: i32) -> Result<i64> {
        sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*)
            FROM entry e
            JOIN entry_tag et ON e.id = et.entry_id
            WHERE et.tag_id = ?
            "#,
        )
        .bind(tag_id)
        .fetch_one(self.pool)
        .await
    }
}
