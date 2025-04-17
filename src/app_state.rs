use sqlx::postgres::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub pool: Arc<PgPool>,
}
