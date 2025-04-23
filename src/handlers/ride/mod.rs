mod accept;
mod create;
mod complete;

use crate::AppState;
use axum::routing::post;
use axum::Router;

pub fn get_router(app_state: &AppState) -> Router {
    Router::new()
        .route("/create", post(create::handler))
        .route("/accept", post(accept::handler))
        .route("/complete", post(complete::handler))
        .with_state(app_state.clone())
}
