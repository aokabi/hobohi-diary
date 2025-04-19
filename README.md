# ほぼ日だいあり

シンプルな日記アプリケーション。Rust（Axum）バックエンドとNext.jsフロントエンドで実装されています。

## プロジェクト構成

- `backend/`: Rust + Axumで実装されたAPIサーバー
- `frontend/`: Next.jsで実装されたフロントエンド

## 開発環境のセットアップ

### Docker Composeを使用する方法（推奨）

Docker Composeを使用すると、バックエンド、フロントエンド、データベースを一度に起動できます。

1. Docker Composeを起動

```bash
docker compose up
```

または、バックグラウンドで実行する場合：

```bash
docker compose up -d
```

2. アクセス

- フロントエンド: http://localhost:3000
- バックエンドAPI: http://localhost:9001/api

3. 停止

```bash
docker compose down
```

データベースのデータを削除する場合：

```bash
docker compose down -v
```

### 個別に起動する方法

#### バックエンド（Rust + Axum）

1. 必要な環境変数を設定

```
DB_USER=root
DB_PASSWORD=
DB_HOST=localhost
DB_PORT=3306
DB_NAME=diary
RUST_LOG=info
```

2. バックエンドの起動

```bash
cd backend
cargo run
```

サーバーは http://localhost:9001 で起動します。

#### フロントエンド（Next.js）

1. 必要な環境変数を設定

```
NEXT_PUBLIC_API_URL=http://localhost:9001/api
```

2. フロントエンドの起動

```bash
cd frontend
npm run dev
```

フロントエンドは http://localhost:3000 で起動します。

## API エンドポイント

- `GET /api/entries?page=1` - 日記エントリの一覧取得（ページネーション付き）
- `POST /api/entries` - 新しい日記エントリの作成
- `GET /api/entries/count` - 日記エントリの総数取得

## 技術スタック

### バックエンド
- Rust
- Axum（Webフレームワーク）
- SQLx（データベースクライアント）
- MySQL（データベース）

### フロントエンド
- Next.js
- React
- TypeScript
- Tailwind CSS
