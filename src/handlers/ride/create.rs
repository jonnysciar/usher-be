use crate::app_state::AppState;
use crate::handlers::UserClaim;
use crate::response::{Error, ErrorKind, Success};
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::Result,
};
use serde::Deserialize;

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
    user_claim: UserClaim,
    State(state): State<AppState>,
    Json(payload): Json<Payload>,
) -> Result<(StatusCode, Json<Success>)> {
    if user_claim.driver {
        return Err(Error::new(ErrorKind::Unauthorized).into());
    }

    let _ = sqlx::query(INSERT_RIDE_QUERY)
        .bind(user_claim.id)
        .bind(payload.start_lat)
        .bind(payload.start_lon)
        .bind(payload.end_lat)
        .bind(payload.end_lon)
        .execute(state.pool.as_ref())
        .await
        .map_err(|_| Error::new(ErrorKind::Server))?;

    Ok((StatusCode::OK, Json::from(Success::new())))
}
