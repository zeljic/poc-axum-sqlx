use std::error::Error;
use crate::app::App;

mod app;
mod config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	App::new().await?.serve().await
}
