use super::User;
use crate::app_state::AppState;
use crate::jwt_controller::RenewClaims;
use crate::response::{Error, ErrorKind, SuccessWithPayload};
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::Result,
};
use serde::Serialize;
use sqlx::types::Uuid;

const SELECT_USER_QUERY: &str = "SELECT * FROM users WHERE id = $1";

#[derive(Debug, Serialize)]
pub struct ReplyPayload {
    renew_token: String,
    token: String,
}

//TODO: Add log
pub async fn handler(
    renew_claims: RenewClaims,
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<SuccessWithPayload<ReplyPayload>>)> {
    let uuid =
        Uuid::try_parse(&renew_claims.sub).map_err(|_| Error::new(ErrorKind::InvalidToken))?;
    let user: User = sqlx::query_as(SELECT_USER_QUERY)
        .bind(uuid)
        .fetch_one(state.pool.as_ref())
        .await
        .map_err(|e| {
            Error::new(match e {
                sqlx::Error::RowNotFound => ErrorKind::LoginFailed,
                _ => ErrorKind::DbConnection,
            })
        })?;

    let reply = ReplyPayload {
        renew_token: state.jwt_controller.encode_renew_token(&user)?,
        token: state.jwt_controller.encode_access_token(user)?,
    };

    Ok((StatusCode::OK, Json::from(SuccessWithPayload::new(reply))))
}
