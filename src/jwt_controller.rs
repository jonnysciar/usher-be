use crate::handlers::{user::User, UserClaim};
use crate::response::{Error, ErrorKind};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use std::{num::ParseIntError, usize};

const SECS_DAY: usize = 86400;
const SECS_90_DAY: usize = SECS_DAY * 30;

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessClaims {
    sub: String, // User id
    exp: usize,
    driver: bool,
}

impl From<&User> for AccessClaims {
    fn from(value: &User) -> Self {
        Self {
            sub: value.id.as_hyphenated().to_string(),
            exp: 0,
            driver: value.driver,
        }
    }
}

pub struct JwtController {
    enc_key: EncodingKey,
    dec_key: DecodingKey,
}

impl JwtController {
    pub fn new() -> Self {
        let jwt_secret = dotenvy::var("JWT_SECRET").expect("Could not start without JWT_SECRET!");
        Self {
            enc_key: EncodingKey::from_secret(&jwt_secret.as_bytes()),
            dec_key: DecodingKey::from_secret(&jwt_secret.as_bytes()),
        }
    }

    pub fn encode_access_token(&self, user: &User) -> Result<String, Error> {
        let mut claims: AccessClaims = user.into();
        claims.exp = jsonwebtoken::get_current_timestamp() as usize + SECS_DAY;
        jsonwebtoken::encode(&Header::default(), &claims, &self.enc_key).map_err(|e| e.into())
    }

    pub fn decode_access_token(&self, token: &str) -> Result<UserClaim, Error> {
        let res =
            jsonwebtoken::decode::<AccessClaims>(token, &self.dec_key, &Validation::default())
                .map_err(|e| e.into())?;
        Uuid::try_parse(&res.claims.sub)
            .map(|u| {
                UserClaim {
                    id: u,
                    driver: res.claims.driver,
                }
            })
            .map_err(|_| Error::new(ErrorKind::InvalidToken))
    }
}

impl Into<Error> for ParseIntError {
    fn into(self) -> Error {
        Error::new(ErrorKind::InvalidToken)
    }
}

impl Into<Error> for jsonwebtoken::errors::Error {
    fn into(self) -> Error {
        Error::new(ErrorKind::InvalidToken)
    }
}
