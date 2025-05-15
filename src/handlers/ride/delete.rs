use crate::app_state::AppState;
use crate::jwt_controller::AccessClaims;
use crate::response::{Error, ErrorKind};
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::Result,
};
use serde::Deserialize;
use sqlx::types::Uuid;

const UPDATE_RIDE_QUERY: &str =
    "UPDATE rides SET state = 'DELETED', ended_at = NOW() WHERE id = $1 AND (state = 'RUNNING' OR state = 'CREATED') AND (driver_id = $2 OR user_id = $2)";

#[derive(Debug, Deserialize)]
pub struct Payload {
    ride_id: String,
}

//TODO: Add log
pub async fn handler(
    access_claims: AccessClaims,
    State(state): State<AppState>,
    Json(payload): Json<Payload>,
) -> Result<StatusCode> {
    let uuid =
        Uuid::try_parse(&access_claims.sub).map_err(|_| Error::new(ErrorKind::InvalidToken))?;
    let ride_uuid =
        Uuid::try_parse(&payload.ride_id).map_err(|_| Error::new(ErrorKind::InvalidRideId))?;

    let n_rows = sqlx::query(UPDATE_RIDE_QUERY)
        .bind(ride_uuid)
        .bind(uuid)
        .execute(state.pool.as_ref())
        .await
        .map_err(|_| Error::new(ErrorKind::Server))?
        .rows_affected();

    if n_rows != 1 {
        return Err(Error::new(ErrorKind::FailedRideOperation).into());
    }

    Ok(StatusCode::OK)
}
