use crate::jwt_controller::JwtController;
use sqlx::postgres::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub pool: Arc<PgPool>,
    pub jwt_controller: Arc<JwtController>,
}
