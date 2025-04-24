use crate::app_state::AppState;
use crate::jwt_controller::AccessClaims;
use crate::response::{Error, ErrorKind, SuccessWithPayload};
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::Result,
};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

const INSERT_RIDE_QUERY: &str = "INSERT INTO rides(user_id, start_lat, start_lon, end_lat, end_lon)
                                 VALUES ($1, $2, $3, $4, $5)
                                 RETURNING id";

#[derive(Debug, Deserialize)]
pub struct Payload {
    start_lat: f64,
    start_lon: f64,
    end_lat: f64,
    end_lon: f64,
}

#[derive(Debug, Serialize)]
pub struct ReplayPayload {
    ride_id: String,
}

//TODO: Add log
pub async fn handler(
    access_claims: AccessClaims,
    State(state): State<AppState>,
    Json(payload): Json<Payload>,
) -> Result<(StatusCode, Json<SuccessWithPayload<ReplayPayload>>)> {
    if access_claims.driver {
        return Err(Error::new(ErrorKind::Unauthorized).into());
    }

    let uuid =
        Uuid::try_parse(&access_claims.sub).map_err(|_| Error::new(ErrorKind::InvalidToken))?;

    let ride_id: Option<Uuid> = sqlx::query_scalar(INSERT_RIDE_QUERY)
        .bind(uuid)
        .bind(payload.start_lat)
        .bind(payload.start_lon)
        .bind(payload.end_lat)
        .bind(payload.end_lon)
        .fetch_optional(state.pool.as_ref())
        .await
        .map_err(|_| Error::new(ErrorKind::Server))?;

    ride_id
        .map(|r| {
            (
                StatusCode::OK,
                Json::from(SuccessWithPayload::new(ReplayPayload {
                    ride_id: r.hyphenated().to_string(),
                })),
            )
        })
        .ok_or(Error::new(ErrorKind::FailedRideOperation).into())
}
