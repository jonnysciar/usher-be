use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Success {
    success: bool,
}

impl Success {
    pub fn new() -> Self {
        Self { success: true }
    }
}

#[derive(Debug, Serialize)]
pub struct SuccessWithPayload<T> {
    success: bool,
    payload: T,
}

impl<T> SuccessWithPayload<T> {
    pub fn new(payload: T) -> Self {
        Self {
            success: true,
            payload,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Error {
    success: bool,
    error_kind: ErrorKind,
}

impl Error {
    pub fn new(error_kind: ErrorKind) -> Self {
        Self {
            success: false,
            error_kind,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (
            self.error_kind.get_status_code(),
            Json::from(Error::from(self)),
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
}

impl ErrorKind {
    fn get_status_code(&self) -> StatusCode {
        match self {
            Self::DbConnection => StatusCode::BAD_GATEWAY,
            Self::Server => StatusCode::INTERNAL_SERVER_ERROR,
            Self::InvalidEmailAddress | Self::EmailAlreadyUsed | Self::LoginFailed => {
                StatusCode::BAD_REQUEST
            }
        }
    }
}
