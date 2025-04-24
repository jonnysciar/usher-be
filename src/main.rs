mod app_state;
mod handlers;
mod jwt_controller;
mod response;

use app_state::AppState;
use axum::Router;
use handlers::{review, ride, user};
use jwt_controller::JwtController;
use sqlx::postgres::PgPool;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::SimpleLogger::new()
        .with_colors(true)
        .with_utc_timestamps()
        .with_level(log::LevelFilter::Trace)
        .env()
        .init()
        .unwrap();

    let app_state = AppState {
        pool: Arc::new(PgPool::connect(&dotenvy::var("DATABASE_URL")?).await?),
        jwt_controller: Arc::new(JwtController::new()),
    };

    let app = Router::new()
        .nest("/user", user::get_router(&app_state))
        .nest("/ride", ride::get_router(&app_state))
        .nest("/review", review::get_router(&app_state));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    Ok(axum::serve(listener, app).await?)
}
