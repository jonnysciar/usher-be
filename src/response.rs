use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Error {
    error_kind: ErrorKind,
}

impl Error {
    pub fn new(error_kind: ErrorKind) -> Self {
        Self {
            error_kind,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (
            self.error_kind.get_status_code(),
            Json::from(self),
        )
            .into_response()
    }
}

#[derive(Debug, Serialize)]
pub enum ErrorKind {
    DbConnection,
    Server,
    InvalidEmailAddress,
    EmailAlreadyUsed,
    LoginFailed,
    Unauthorized,
    InvalidToken,
    InvalidRideId,
    InvalidReview,
    FailedRideOperation,
}

impl ErrorKind {
    fn get_status_code(&self) -> StatusCode {
        match self {
            Self::DbConnection => StatusCode::BAD_GATEWAY,
            Self::Server => StatusCode::INTERNAL_SERVER_ERROR,
            Self::InvalidEmailAddress
            | Self::EmailAlreadyUsed
            | Self::LoginFailed
            | Self::InvalidToken
            | Self::InvalidRideId
            | Self::InvalidReview
            | Self::FailedRideOperation => StatusCode::BAD_REQUEST,
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
        }
    }
}
