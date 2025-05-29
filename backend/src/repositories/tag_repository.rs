use sqlx::{MySqlPool, Result, Transaction, MySql};
use crate::models::tag::Tag;

pub struct TagRepository<'a> {
    // poolは直接使わないが、将来的な拡張のために残すことも検討
    // pool: &'a MySqlPool,
    _marker: std::marker::PhantomData<&'a ()>, // ライフタイム'aを使用するためのマーカー
}

impl<'a> TagRepository<'a> {
    // pub fn new(pool: &'a MySqlPool) -> Self {
    //     Self { pool }
    // }
    pub fn new() -> Self {
        Self { _marker: std::marker::PhantomData }
    }

    /// 指定されたエントリIDに関連付けられたタグを取得します。
    pub async fn find_tags_for_entry(&self, pool: &MySqlPool, entry_id: i32) -> Result<Vec<Tag>> {
        sqlx::query_as::<_, Tag>(
            r#"
            SELECT t.id, t.name
            FROM tag t
            JOIN entry_tag et ON t.id = et.tag_id
            WHERE et.entry_id = ?
            ORDER BY t.name
            "#,
        )
        .bind(entry_id)
        .fetch_all(pool) // ここではpoolを直接使う
        .await
    }

    /// タグ名でタグを検索し、存在しない場合は作成します（トランザクション内）。
    /// 戻り値はタグのIDです。
    pub async fn find_or_create_tag_tx(
        tx: &mut Transaction<'_, MySql>,
        tag_name: &str,
    ) -> Result<i32> {
        // タグが存在するか確認
        let tag_id = match sqlx::query_scalar::<_, i32>("SELECT id FROM tag WHERE name = ?")
            .bind(tag_name)
            .fetch_optional(&mut **tx) // Dereference tx
            .await?
        {
            Some(id) => id, // タグが存在する場合
            None => {
                // タグが存在しない場合は作成
                let result = sqlx::query("INSERT INTO tag (name) VALUES (?)")
                    .bind(tag_name)
                    .execute(&mut **tx) // Dereference tx
                    .await?;
                result.last_insert_id() as i32
            }
        };
        Ok(tag_id)
    }

    /// エントリとタグを関連付けます（トランザクション内）。
    pub async fn associate_entry_with_tag_tx(
        tx: &mut Transaction<'_, MySql>,
        entry_id: i32,
        tag_id: i32,
    ) -> Result<()> {
        sqlx::query("INSERT INTO entry_tag (entry_id, tag_id) VALUES (?, ?)")
            .bind(entry_id)
            .bind(tag_id)
            .execute(&mut **tx) // Dereference tx
            .await?;
        Ok(())
    }

    /// すべてのタグを取得します。
    pub async fn find_all_tags(&self, pool: &MySqlPool) -> Result<Vec<Tag>> {
        sqlx::query_as::<_, Tag>("SELECT id, name FROM tag ORDER BY name")
            .fetch_all(pool)
            .await
    }
}
