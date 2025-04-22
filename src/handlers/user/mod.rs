mod info;
mod login;
mod signup;

use crate::AppState;
use axum::routing::{get, post};
use axum::Router;
use sqlx::types::Uuid;

#[derive(sqlx::FromRow, Debug, PartialEq, Eq)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub last_name: String,
    pub driver: bool,
    pub hashed_password: String,
}

pub fn get_router(app_state: &AppState) -> Router {
    Router::new()
        .route("/signup", post(signup::handler))
        .route("/login", post(login::handler))
        .route("/info", get(info::handler))
        .with_state(app_state.clone())
}
