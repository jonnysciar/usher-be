mod add;

use crate::AppState;
use axum::routing::post;
use axum::Router;

pub fn get_router(app_state: &AppState) -> Router {
    Router::new()
        .route("/add", post(add::handler))
        .with_state(app_state.clone())
}