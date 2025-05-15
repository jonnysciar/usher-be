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
    "UPDATE rides SET driver_id = $1, state = 'RUNNING' WHERE id = $2 AND state = 'CREATED'";

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
    if !access_claims.driver {
        return Err(Error::new(ErrorKind::Unauthorized).into());
    }

    let uuid =
        Uuid::try_parse(&access_claims.sub).map_err(|_| Error::new(ErrorKind::InvalidToken))?;
    let ride_uuid =
        Uuid::try_parse(&payload.ride_id).map_err(|_| Error::new(ErrorKind::InvalidRideId))?;

    let n_rows = sqlx::query(UPDATE_RIDE_QUERY)
        .bind(uuid)
        .bind(ride_uuid)
        .execute(state.pool.as_ref())
        .await
        .map_err(|_| Error::new(ErrorKind::Server))?
        .rows_affected();

    if n_rows != 1 {
        return Err(Error::new(ErrorKind::FailedRideOperation).into());
    }

    Ok(StatusCode::OK)
}
