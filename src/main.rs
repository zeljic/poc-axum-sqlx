use crate::app::App;
use std::error::Error;

mod app;
mod config;
mod error;
mod models;
mod routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    App::new().await?.serve().await
}
