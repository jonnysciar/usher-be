use super::{User, UserId};
use crate::app_state::AppState;
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
    name: String,
    last_name: String,
}

//TODO: Add log
pub async fn handler(
    user_id: UserId,
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<SuccessWithPayload<ReplyPayload>>)> {
    let user: User = sqlx::query_as(SELECT_USER_QUERY)
        .bind(user_id.0)
        .fetch_one(state.pool.as_ref())
        .await
        .map_err(|e| {
            Error::new(match e {
                sqlx::Error::RowNotFound => ErrorKind::LoginFailed,
                _ => ErrorKind::DbConnection,
            })
        })?;

    let reply = ReplyPayload {
        name: user.name,
        last_name: user.last_name,
    };

    Ok((StatusCode::OK, Json::from(SuccessWithPayload::new(reply))))
}
