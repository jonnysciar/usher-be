mod app_state;
mod handlers;
mod response;

use app_state::AppState;
use axum::{routing::post, Router};
use handlers::{create_user, login};
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
    };

    let user_router = Router::new()
        .route("/create", post(create_user::handler))
        .route("/login", post(login::handler))
        .with_state(app_state);

    let app = Router::new().nest("/user", user_router);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    Ok(axum::serve(listener, app).await?)
}
