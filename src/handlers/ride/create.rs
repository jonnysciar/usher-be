use crate::app_state::AppState;
use crate::jwt_controller::AccessClaims;
use crate::response::{Error, ErrorKind, Success};
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::Result,
};
use serde::Deserialize;
use sqlx::types::Uuid;

const INSERT_RIDE_QUERY: &str = "INSERT INTO rides(user_id, start_lat, start_lon, end_lat, end_lon) VALUES ($1, $2, $3, $4, $5)";

#[derive(Debug, Deserialize)]
pub struct Payload {
    start_lat: f64,
    start_lon: f64,
    end_lat: f64,
    end_lon: f64,
}

//TODO: Add log
pub async fn handler(
    access_claims: AccessClaims,
    State(state): State<AppState>,
    Json(payload): Json<Payload>,
) -> Result<(StatusCode, Json<Success>)> {
    if access_claims.driver {
        return Err(Error::new(ErrorKind::Unauthorized).into());
    }

    let uuid =
        Uuid::try_parse(&access_claims.sub).map_err(|_| Error::new(ErrorKind::InvalidToken))?;

    let n_rows = sqlx::query(INSERT_RIDE_QUERY)
        .bind(uuid)
        .bind(payload.start_lat)
        .bind(payload.start_lon)
        .bind(payload.end_lat)
        .bind(payload.end_lon)
        .execute(state.pool.as_ref())
        .await
        .map_err(|_| Error::new(ErrorKind::Server))?
        .rows_affected();

    if n_rows != 1 {
        return Err(Error::new(ErrorKind::FailedRideOperation).into());
    }

    Ok((StatusCode::OK, Json::from(Success::new())))
}
