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

const CHECK_RIDE_QUERY: &str = "SELECT EXISTS (
                    SELECT * FROM rides
                    WHERE id = $1 AND state = 'COMPLETED' AND (user_id = $2 OR driver_id = $2))";

const UPSERT_USER_REVIEW_QUERY: &str =
    "INSERT INTO reviews(ride_id, user_review, user_stars, user_review_created_at)
     VALUES ($1, $2, $3, NOW())
     ON CONFLICT (ride_id)
     DO UPDATE SET user_review = excluded.user_review,
                   user_stars = excluded.user_stars,
                   user_review_created_at = excluded.user_review_created_at
     WHERE reviews.user_review IS NULL AND reviews.user_stars IS NULL";

const UPSERT_DRIVER_REVIEW_QUERY: &str =
    "INSERT INTO reviews(ride_id, driver_review, driver_stars, driver_review_created_at)
     VALUES ($1, $2, $3, NOW())
     ON CONFLICT (ride_id)
     DO UPDATE SET driver_review = excluded.driver_review,
                   driver_stars = excluded.driver_stars,
                   driver_review_created_at = excluded.driver_review_created_at
     WHERE reviews.driver_review IS NULL AND reviews.driver_stars IS NULL";

#[derive(Debug, Deserialize)]
pub struct Payload {
    ride_id: String,
    review: String,
    stars: i32,
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

    let is_my_ride: bool = sqlx::query_scalar(CHECK_RIDE_QUERY)
        .bind(ride_uuid)
        .bind(uuid)
        .fetch_one(state.pool.as_ref())
        .await
        .map_err(|_| Error::new(ErrorKind::Server))?;

    if !is_my_ride {
        return Err(Error::new(ErrorKind::InvalidRideId).into());
    }

    if payload.stars < 1 || payload.stars > 5 {
        return Err(Error::new(ErrorKind::InvalidReview).into());
    }

    let query = match access_claims.driver {
        true => sqlx::query(UPSERT_USER_REVIEW_QUERY),
        false => sqlx::query(UPSERT_DRIVER_REVIEW_QUERY),
    };
    let n_rows = query
        .bind(ride_uuid)
        .bind(payload.review)
        .bind(payload.stars)
        .execute(state.pool.as_ref())
        .await
        .map_err(|_| Error::new(ErrorKind::Server))?
        .rows_affected();

    if n_rows != 1 {
        return Err(Error::new(ErrorKind::InvalidReview).into());
    }

    Ok(StatusCode::OK)
}
