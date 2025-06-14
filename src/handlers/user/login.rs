use super::User;
use crate::app_state::AppState;
use crate::response::{Error, ErrorKind};
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::Result,
};
use serde::{Deserialize, Serialize};

const SELECT_USER_QUERY: &str = "SELECT * FROM users WHERE email = $1";

#[derive(Debug, Deserialize)]
pub struct Payload {
    email: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct ReplyPayload {
    renew_token: String,
    token: String,
}

//TODO: Add log
pub async fn handler(
    State(state): State<AppState>,
    Json(payload): Json<Payload>,
) -> Result<(StatusCode, Json<ReplyPayload>)> {
    let user: User = sqlx::query_as(SELECT_USER_QUERY)
        .bind(payload.email)
        .fetch_one(state.pool.as_ref())
        .await
        .map_err(|e| {
            Error::new(match e {
                sqlx::Error::RowNotFound => ErrorKind::LoginFailed,
                _ => ErrorKind::DbConnection,
            })
        })?;

    if !bcrypt::verify(payload.password, &user.hashed_password)
        .map_err(|_| Error::new(ErrorKind::Server))?
    {
        return Err(Error::new(ErrorKind::LoginFailed).into());
    }

    let reply = ReplyPayload {
        renew_token: state.jwt_controller.encode_renew_token(&user)?,
        token: state.jwt_controller.encode_access_token(user)?,
    };

    Ok((StatusCode::OK, Json::from(reply)))
}
