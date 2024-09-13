use std::error::Error;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Router, ServiceExt};
use axum::routing::get;
use serde_json::json;
use sqlx::{Pool, Sqlite};
use crate::config::Config;

pub struct App {
	config: Config,
	db: Pool<Sqlite>,
}

async fn get_db_pool(config: &Config) -> Result<sqlx::SqlitePool, sqlx::Error> {
	sqlx::sqlite::SqlitePoolOptions::new()
		.max_connections(config.db_max_connections)
		.min_connections(config.db_min_connections)
		.connect(&config.db_url)
		.await
}

impl App {
	pub async fn new() -> Result<Self, Box<dyn Error>> {
		dotenvy::dotenv().ok();

		let config = Config::new();
		let db = get_db_pool(&config).await?;

		Ok(Self { config, db })
	}

	pub async fn serve(self) -> Result<(), Box<dyn Error>> {
		let router = Router::new()
			.route("/", get(index))
			.route("/health", get(health))
			.fallback(handle_404);

		let listener = tokio::net::TcpListener::bind((self.config.server_host.as_str(), self.config.server_port)).await?;

		axum::serve(listener, router.into_make_service()).await?;

		Ok(())
	}
}

async fn handle_404() -> impl IntoResponse {
	(StatusCode::NOT_FOUND, "")
}

async fn index() -> impl IntoResponse {
	(StatusCode::OK, "zdravo svete!")
}

async fn health() -> impl IntoResponse {
	(StatusCode::OK, axum::response::Json(json!({ "status": "ok" })))
}