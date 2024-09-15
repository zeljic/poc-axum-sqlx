use axum::Router;
use crate::app::AppState;

mod users;

pub async fn router(app_state: AppState) -> Result<Router, Box<dyn std::error::Error>> {
	let router = Router::new()
		.nest("/users", users::router(AppState::clone(&app_state)).await?);

	Ok(router)
}