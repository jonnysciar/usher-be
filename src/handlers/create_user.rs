use crate::app_state::AppState;
use crate::response::{Error, ErrorKind, Success};
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::Result,
};
use email_address::EmailAddress;
use serde::Deserialize;
use std::str::FromStr;

const INSERT_USER_QUERY: &str = "INSERT INTO users(email, name, last_name, driver, hashed_password) VALUES ($1, $2, $3, $4, $5)";

#[derive(Debug, Deserialize)]
pub struct Payload {
    email: String,
    name: String,
    last_name: String,
    driver: bool,
    password: String,
}

//TODO: Add log
pub async fn handler(
    State(state): State<AppState>,
    Json(payload): Json<Payload>,
) -> Result<(StatusCode, Json<Success>)> {
    let email = EmailAddress::from_str(&payload.email)
        .map_err(|_| Error::new(ErrorKind::InvalidEmailAddress))?;

    let hashed_pwd = bcrypt::hash(payload.password, bcrypt::DEFAULT_COST)
        .map_err(|_| Error::new(ErrorKind::Server))?;

    let _ = sqlx::query(INSERT_USER_QUERY)
        .bind(email.as_str())
        .bind(payload.name)
        .bind(payload.last_name)
        .bind(payload.driver)
        .bind(hashed_pwd)
        .execute(state.pool.as_ref())
        .await
        .map_err(|e| {
            Error::new(match e {
                sqlx::Error::Database(e) => match e.kind() {
                    sqlx::error::ErrorKind::UniqueViolation => ErrorKind::EmailAlreadyUsed,
                    _ => ErrorKind::InvalidEmailAddress,
                },
                _ => ErrorKind::DbConnection,
            })
        })?;
    Ok((StatusCode::CREATED, Json::from(Success::new())))
}
