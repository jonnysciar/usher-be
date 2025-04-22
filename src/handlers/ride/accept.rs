use crate::app_state::AppState;
use crate::handlers::UserClaim;
use crate::response::{Error, ErrorKind, Success};
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::Result,
};
use serde::Deserialize;
use sqlx::types::Uuid;

const INSERT_RIDE_QUERY: &str = "UPDATE rides SET driver_id = $1, state = 'RUNNING' WHERE id = $2";

#[derive(Debug, Deserialize)]
pub struct Payload {
    ride_id: String,
}

//TODO: Add log
pub async fn handler(
    user_claim: UserClaim,
    State(state): State<AppState>,
    Json(payload): Json<Payload>,
) -> Result<(StatusCode, Json<Success>)> {
    if !user_claim.driver {
        return Err(Error::new(ErrorKind::Unauthorized).into());
    }

    let ride_uuid =
        Uuid::try_parse(&payload.ride_id).map_err(|_| Error::new(ErrorKind::InvalidRideId))?;

    let _ = sqlx::query(INSERT_RIDE_QUERY)
        .bind(user_claim.id)
        .bind(ride_uuid)
        .execute(state.pool.as_ref())
        .await
        .map_err(|_| Error::new(ErrorKind::Server))?;

    Ok((StatusCode::OK, Json::from(Success::new())))
}
