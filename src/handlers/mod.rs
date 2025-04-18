pub mod login;
pub mod signup;
pub mod userinfo;

use axum::{extract::FromRequestParts, http::request::Parts, RequestPartsExt};
use axum_extra::{headers::{authorization::Bearer, Authorization}, TypedHeader};

use crate::response::{Error, ErrorKind};
use crate::AppState;

#[derive(sqlx::FromRow, Debug, PartialEq, Eq)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub last_name: String,
    pub driver: bool,
    pub hashed_password: String,
}

pub struct UserId(pub i32);

impl FromRequestParts<AppState> for UserId
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| Error::new(ErrorKind::InvalidToken))?;

        state.jwt_controller.decode_access_token(bearer.token()).map_err(|e| e.into())
    }
}