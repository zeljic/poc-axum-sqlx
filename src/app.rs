use std::error::Error;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Router, ServiceExt};
use axum::extract::FromRef;
use axum::routing::get;
use serde_json::json;
use sqlx::{Pool, Sqlite};
use crate::config::Config;

pub struct AppState {
	pub config: Config,
	pub db: Pool<Sqlite>,
}

impl Clone for AppState {
	fn clone(&self) -> Self {
		Self {
			config: self.config.clone(),
			db: self.db.clone(),
		}
	}
}

impl FromRef<AppState> for Config {
	fn from_ref(state: &AppState) -> Self {
		state.config.clone()
	}
}

impl FromRef<AppState> for Pool<Sqlite> {
	fn from_ref(state: &AppState) -> Self {
		state.db.clone()
	}
}

pub struct App {
	app_state: AppState,
}

impl App {
	pub async fn new() -> Result<Self, Box<dyn Error>> {
		dotenvy::dotenv().ok();

		let config = Config::new();
		let db = sqlx::sqlite::SqlitePoolOptions::new()
			.max_connections(config.db_max_connections)
			.min_connections(config.db_min_connections)
			.connect_lazy(&config.db_url)?;

		Ok(Self {
			app_state: AppState {
				config,
				db,
			}
		})
	}

	pub async fn serve(self) -> Result<(), Box<dyn Error>> {
		let app_state = self.app_state.clone();
		let config = self.app_state.config.clone();

		let router = Router::new()
			.route("/", get(index))
			.route("/health", get(health))
			.nest("/", crate::routes::router(app_state).await?)
			.fallback(handle_404);

		let listener = tokio::net::TcpListener::bind((config.server_host, config.server_port)).await?;

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