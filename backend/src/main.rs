mod db;
mod handlers;
mod models;
mod routes;

use axum::http::{HeaderName, HeaderValue, Method};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 環境変数の読み込み
    dotenv::dotenv().ok();

    // ロギングの初期化
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 環境変数からDB接続情報を取得
    let db_user = std::env::var("DB_USER").unwrap_or_default();
    let db_password = std::env::var("DB_PASSWORD").unwrap_or_default();
    let db_host = std::env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string());
    let db_port = std::env::var("DB_PORT").unwrap_or_else(|_| "3306".to_string());
    let db_name = std::env::var("DB_NAME").unwrap_or_default();

    let database_url = format!(
        "mysql://{}:{}@{}:{}/{}",
        db_user, db_password, db_host, db_port, db_name
    );

    // データベース接続プールの作成
    let pool = db::connection::create_pool(&database_url).await?;

    let origins: Vec<HeaderValue> = std::env::var("ALLOWED_ORIGINS")
        .unwrap_or_default()
        .split(",")
        .map(|s| s.trim().parse().unwrap())
        .collect();

    // CORSの設定
    let cors = CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([HeaderName::from_static("content-type")]);

    // ルーターの作成
    let app = routes::create_router(pool).layer(cors);

    // サーバーアドレスの設定
    let addr = SocketAddr::from(([0, 0, 0, 0], 9001));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("listening on {}", addr);

    // サーバーの起動
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
