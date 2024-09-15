use std::error::Error;
use std::str::FromStr;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Router};
use axum::extract::FromRef;
use axum::routing::get;
use serde_json::json;
use sqlx::{Pool, Sqlite};
use tracing_appender::non_blocking::WorkerGuard;
use crate::config::Config;

/// Application state is a shared state that can be accessed by all handlers
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

/// Implement the `FromRef` trait for the `AppState` struct
/// This allows us to convert a reference to the `AppState` struct into a Config struct
impl FromRef<AppState> for Config {
	fn from_ref(state: &AppState) -> Self {
		state.config.clone()
	}
}

/// Implement the `FromRef` trait for the `AppState` struct
/// This allows us to convert a reference to the `AppState` struct into a Sqlite pool
impl FromRef<AppState> for Pool<Sqlite> {
	fn from_ref(state: &AppState) -> Self {
		state.db.clone()
	}
}

/// The main application struct
/// This struct holds the application state
/// and is responsible for starting the server
/// and serving the application.
pub struct App {
	app_state: AppState,
}

impl App {
	/// Create a new instance of the application
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

	/// Start the server and serve the application
	pub async fn serve(self) -> Result<(), Box<dyn Error>> {
		let app_state = self.app_state.clone();
		let config = self.app_state.config.clone();

		let _guard = init_tracing(&config);

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

/// Handle 404 errors
/// This function is called when a route is not found
async fn handle_404() -> impl IntoResponse {
	(StatusCode::NOT_FOUND, "")
}

/// The index route handler
/// This function is called when the root path is requested
async fn index() -> impl IntoResponse {
	(StatusCode::OK, "zdravo svete!")
}

/// The health route handler
/// This function is called when the /health path is requested
async fn health() -> impl IntoResponse {
	(StatusCode::OK, axum::response::Json(json!({ "status": "ok" })))
}

/// Initialize tracing in release mode
#[cfg(not(debug_assertions))]
fn init_tracing(config: &Config) -> WorkerGuard {
	let log_file_path = std::path::Path::new(&config.log_file_path);

	let log_file = tracing_appender::rolling::never(
		log_file_path.parent().expect("Invalid log file path"),
		log_file_path.file_name().expect("Invalid log file path"),
	);

	let (non_blocking_writer, guard) = tracing_appender::non_blocking(log_file);

	tracing_subscriber::fmt()
		.with_writer(non_blocking_writer)
		.with_max_level(tracing::Level::from_str(config.log_level.as_str()).unwrap_or(tracing::Level::ERROR))
		.with_ansi(false)
		.compact()
		.init();

	guard
}

/// Initialize tracing in debug mode
#[cfg(debug_assertions)]
fn init_tracing(config: &Config) -> WorkerGuard {
	let (non_blocking_writer, guard) = tracing_appender::non_blocking(std::io::stdout());

	tracing_subscriber::fmt()
		.with_writer(non_blocking_writer)
		.with_max_level(tracing::Level::from_str(config.log_level.as_str()).unwrap_or(tracing::Level::DEBUG))
		.compact()
		.init();

	guard
}