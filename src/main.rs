mod app_state;
mod handlers;
mod response;
mod jwt_controller;

use app_state::AppState;
use axum::{routing::{get, post}, Router};
use handlers::{login, signup, userinfo};
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

    let user_router = Router::new()
        .route("/signup", post(signup::handler))
        .route("/login", post(login::handler))
        .route("/info", get(userinfo::handler))
        .with_state(app_state);

    let app = Router::new().nest("/user", user_router);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    Ok(axum::serve(listener, app).await?)
}
