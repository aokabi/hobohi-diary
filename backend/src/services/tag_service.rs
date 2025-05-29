use sqlx::{MySqlPool, Result, Error as SqlxError};
use crate::models::tag::Tag;
use crate::repositories::tag_repository::TagRepository;

// エラー型を定義（サービス層固有のエラーを返す場合）
#[derive(Debug)]
pub enum TagServiceError {
    // 未使用フィールド警告を抑制するため SqlxError を () に変更
    DatabaseError(()),
    // 他のエラーケースがあれば追加
}

// sqlx::ErrorからTagServiceErrorへの変換
impl From<SqlxError> for TagServiceError {
    fn from(err: SqlxError) -> Self {
        // エラーの詳細はここでログに出力するなどしても良い
        tracing::error!("Database error occurred: {:?}", err);
        // 警告抑制のため、具体的なエラー情報は含めずにユニット型を返す
        TagServiceError::DatabaseError(())
    }
}

pub struct TagService<'a> {
    pool: &'a MySqlPool,
    tag_repo: TagRepository<'a>,
}

impl<'a> TagService<'a> {
    pub fn new(pool: &'a MySqlPool) -> Self {
        Self {
            pool,
            tag_repo: TagRepository::new(), // TagRepositoryは現状poolを直接使わない
        }
    }

    /// すべてのタグを取得します。
    pub async fn get_all_tags(&self) -> Result<Vec<Tag>, TagServiceError> {
        let tags = self.tag_repo.find_all_tags(self.pool).await?;
        Ok(tags)
    }

    /// タグ名でタグを検索し、存在しない場合は作成します。
    /// 戻り値は検索または作成されたタグです。
    pub async fn find_or_create_tag(&self, tag_name: &str) -> Result<Tag, TagServiceError> {
        // タグ名が空でないことを確認 (サービス層でのバリデーション)
        if tag_name.trim().is_empty() {
            // ここではカスタムエラーを返す代わりに、リポジトリ層に任せるか、
            // もしくはサービス層固有のバリデーションエラーを定義して返す
            // 今回はリポジトリ層(find_or_create_tag_tx)がエラーを返すことを期待する
             tracing::warn!("Attempted to find or create an empty tag name");
             // return Err(TagServiceError::ValidationError("Tag name cannot be empty".to_string()));
        }

        // トランザクションを開始してタグの検索または作成を行う
        let mut tx = self.pool.begin().await?;
        let tag_id = TagRepository::find_or_create_tag_tx(&mut tx, tag_name).await?;
        tx.commit().await?; // トランザクションをコミット

        // IDが分かったので、完全なTagオブジェクトを取得して返す
        // (find_or_create_tag_txはIDしか返さないため)
        // ここは最適化の余地あり。find_or_create_tag_txがTagを返すように変更するなど。
        // 今回はシンプルに再度取得する。
        let tag = sqlx::query_as::<_, Tag>("SELECT id, name FROM tag WHERE id = ?")
            .bind(tag_id)
            .fetch_one(self.pool)
            .await?; // エラーはFromトレイトによりTagServiceErrorに変換される

        Ok(tag)
    }

    // 必要に応じて他のタグ関連ビジネスロジックを追加
    // 例: タグの更新、削除など
}
