use crate::app_state::AppState;
use crate::handlers::user::User;
use crate::response::{Error, ErrorKind};
use axum::{extract::FromRequestParts, http::request::Parts, RequestPartsExt};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::errors as jwt_errors;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

const SECS_DAY: usize = 86400;
const SECS_30_DAY: usize = SECS_DAY * 30;

pub struct JwtController {
    enc_key: EncodingKey,
    dec_key: DecodingKey,
}

impl JwtController {
    pub fn new() -> Self {
        let jwt_secret = dotenvy::var("JWT_SECRET").expect("Could not start without JWT_SECRET!");
        Self {
            enc_key: EncodingKey::from_secret(jwt_secret.as_bytes()),
            dec_key: DecodingKey::from_secret(jwt_secret.as_bytes()),
        }
    }

    pub fn encode_access_token(&self, user: User) -> Result<String, Error> {
        let mut claims: AccessClaims = user.into();
        claims.exp = jsonwebtoken::get_current_timestamp() as usize + SECS_DAY;
        Ok(jsonwebtoken::encode(
            &Header::default(),
            &claims,
            &self.enc_key,
        )?)
    }

    pub fn decode_access_token(&self, token: &str) -> Result<AccessClaims, Error> {
        Ok(
            jsonwebtoken::decode::<AccessClaims>(token, &self.dec_key, &Validation::default())?
                .claims,
        )
    }

    pub fn encode_renew_token(&self, user: &User) -> Result<String, Error> {
        let mut claims: RenewClaims = user.into();
        claims.exp = jsonwebtoken::get_current_timestamp() as usize + SECS_30_DAY;
        Ok(jsonwebtoken::encode(
            &Header::default(),
            &claims,
            &self.enc_key,
        )?)
    }

    pub fn decode_renew_token(&self, token: &str) -> Result<RenewClaims, Error> {
        Ok(
            jsonwebtoken::decode::<RenewClaims>(token, &self.dec_key, &Validation::default())?
                .claims,
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessClaims {
    pub sub: String, // User id
    exp: usize,
    pub name: String,
    pub last_name: String,
    pub email: String,
    pub driver: bool,
}

impl From<User> for AccessClaims {
    fn from(value: User) -> Self {
        Self {
            sub: value.id.as_hyphenated().to_string(),
            exp: 0,
            name: value.name,
            last_name: value.last_name,
            email: value.email,
            driver: value.driver,
        }
    }
}

impl FromRequestParts<AppState> for AccessClaims {
    type Rejection = Error;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| Error::new(ErrorKind::InvalidToken))?;

        state.jwt_controller.decode_access_token(bearer.token())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RenewClaims {
    pub sub: String, // User id
    exp: usize,
}

impl From<&User> for RenewClaims {
    fn from(value: &User) -> Self {
        Self {
            sub: value.id.as_hyphenated().to_string(),
            exp: 0,
        }
    }
}

impl FromRequestParts<AppState> for RenewClaims {
    type Rejection = Error;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| Error::new(ErrorKind::InvalidToken))?;

        state.jwt_controller.decode_renew_token(bearer.token())
    }
}

impl From<jwt_errors::Error> for Error {
    fn from(value: jwt_errors::Error) -> Self {
        Error::new(match value.kind() {
            jwt_errors::ErrorKind::InvalidRsaKey(_)
            | jwt_errors::ErrorKind::InvalidKeyFormat
            | jwt_errors::ErrorKind::InvalidEcdsaKey => ErrorKind::Server,
            _ => ErrorKind::InvalidToken,
        })
    }
}
