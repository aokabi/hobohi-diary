use sqlx::{MySqlPool, Result, Error as SqlxError};
use crate::models::{
    entry::{Entry, CreateEntryRequest}, // CreateEntryRequest は simple_create で使用
    tag::EntryWithTags, // EntryWithTags は create_entry_with_tags で使用
};
use crate::repositories::{
    diary_repository::DiaryRepository,
    tag_repository::TagRepository,
};
use crate::handlers::diary::{EntriesWithTagsResponse, EntryWithTagsResponse}; // ハンドラーのレスポンス型を再利用

// エラー型を定義（サービス層固有のエラーを返す場合）
#[derive(Debug)]
pub enum DiaryServiceError {
    // 未使用フィールド警告を抑制するため SqlxError を () に変更
    // 未使用バリアント NotFound, ValidationError を削除
    DatabaseError(()),
}

// sqlx::ErrorからDiaryServiceErrorへの変換
impl From<SqlxError> for DiaryServiceError {
    fn from(err: SqlxError) -> Self {
        // エラーの詳細はここでログに出力するなどしても良い
        tracing::error!("Database error occurred: {:?}", err);
        // 警告抑制のため、具体的なエラー情報は含めずにユニット型を返す
        DiaryServiceError::DatabaseError(())
    }
}

pub struct DiaryService<'a> {
    pool: &'a MySqlPool,
    diary_repo: DiaryRepository<'a>,
    tag_repo: TagRepository<'a>,
}

impl<'a> DiaryService<'a> {
    pub fn new(pool: &'a MySqlPool) -> Self {
        Self {
            pool,
            diary_repo: DiaryRepository::new(pool),
            tag_repo: TagRepository::new(), // TagRepositoryは現状poolを直接使わない
        }
    }

    /// エントリ一覧（タグ付き）とページネーション情報を取得します。
    pub async fn get_entries_with_details(
        &self,
        page: u32,
        limit: u32,
    ) -> Result<EntriesWithTagsResponse, DiaryServiceError> {
        let offset = (page - 1) * limit;

        // エントリ取得
        let entries = self.diary_repo.find_entries(limit, offset).await?;
        // 総数取得
        let count = self.diary_repo.count_entries().await?;

        let total_pages = (count as f64 / limit as f64).ceil() as u32;

        // エントリごとにタグを取得
        let mut entries_with_tags_response = Vec::new();
        for entry in entries {
            // TagRepositoryを使ってタグを取得
            let tags = self.tag_repo.find_tags_for_entry(self.pool, entry.id).await?;
            entries_with_tags_response.push(EntryWithTagsResponse {
                id: entry.id,
                content: entry.content.clone(), // 必要に応じてclone
                datetime: entry.datetime,
                tags,
            });
        }

        Ok(EntriesWithTagsResponse {
            entries: entries_with_tags_response,
            total_pages,
            current_page: page,
        })
    }

    /// 新しいエントリとタグを作成します。
    pub async fn create_entry_with_tags(
        &self,
        entry_data: &EntryWithTags, // ハンドラーから渡されるデータ構造
    ) -> Result<(), DiaryServiceError> {
        // トランザクション開始
        let mut tx = self.pool.begin().await?;

        // 1. エントリを作成 (リポジトリを使用)
        let entry_id = DiaryRepository::create_entry_with_tags_tx(&mut tx, &entry_data.content).await?;

        // 2. タグを処理 (リポジトリを使用)
        for tag_name in &entry_data.tags {
            if tag_name.trim().is_empty() {
                continue; // 空のタグはスキップ
            }
            // タグを検索または作成
            let tag_id = TagRepository::find_or_create_tag_tx(&mut tx, tag_name).await?;
            // エントリとタグを関連付け
            TagRepository::associate_entry_with_tag_tx(&mut tx, entry_id, tag_id).await?;
        }

        // トランザクションのコミット
        tx.commit().await?;

        Ok(())
    }

    /// シンプルなエントリを作成します（タグなし）。
    pub async fn create_simple_entry(
        &self,
        request: &CreateEntryRequest,
    ) -> Result<(), DiaryServiceError> {
        self.diary_repo.create_simple_entry(&request.content).await?;
        Ok(())
    }

    /// エントリの総数を取得します。
    pub async fn get_entry_count(&self) -> Result<i64, DiaryServiceError> {
        let count = self.diary_repo.count_entries().await?;
        Ok(count)
    }

    /// タグIDでフィルタリングされたエントリ一覧とページネーション情報を取得します。
    pub async fn get_entries_by_tag(
        &self,
        tag_id: i32,
        page: u32,
        limit: u32,
    ) -> Result<(Vec<Entry>, u32), DiaryServiceError> { // 戻り値をタプルに変更 (EntriesResponseはハンドラー層で構築)
        let offset = (page - 1) * limit;

        // エントリ取得
        let entries = self.diary_repo.find_entries_by_tag(tag_id, limit, offset).await?;
        // 総数取得
        let count = self.diary_repo.count_entries_by_tag(tag_id).await?;

        let total_pages = (count as f64 / limit as f64).ceil() as u32;

        Ok((entries, total_pages))
    }
}
