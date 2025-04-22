pub mod ride;
pub mod user;

use axum::{extract::FromRequestParts, http::request::Parts, RequestPartsExt};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use sqlx::types::Uuid;

use crate::{
    app_state::AppState,
    response::{Error, ErrorKind},
};

pub struct UserClaim {
    pub id: Uuid,
    pub driver: bool,
}

impl FromRequestParts<AppState> for UserClaim {
    type Rejection = Error;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| Error::new(ErrorKind::InvalidToken))?;

        state
            .jwt_controller
            .decode_access_token(bearer.token())
            .map_err(|e| e.into())
    }
}
