use super::User;
use crate::app_state::AppState;
use crate::handlers::UserClaim;
use crate::response::{Error, ErrorKind, SuccessWithPayload};
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::Result,
};
use serde::Serialize;

const SELECT_USER_QUERY: &str = "SELECT * FROM users WHERE id = $1";

#[derive(Debug, Serialize)]
pub struct ReplyPayload {
    email: String,
    name: String,
    last_name: String,
    driver: bool,
}

//TODO: Add log
pub async fn handler(
    user_claim: UserClaim,
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<SuccessWithPayload<ReplyPayload>>)> {
    let user: User = sqlx::query_as(SELECT_USER_QUERY)
        .bind(user_claim.id)
        .fetch_one(state.pool.as_ref())
        .await
        .map_err(|e| {
            Error::new(match e {
                sqlx::Error::RowNotFound => ErrorKind::LoginFailed,
                _ => ErrorKind::DbConnection,
            })
        })?;

    let reply = ReplyPayload {
        email: user.email,
        name: user.name,
        last_name: user.last_name,
        driver: user.driver,
    };

    Ok((StatusCode::OK, Json::from(SuccessWithPayload::new(reply))))
}
