use axum::{http::StatusCode, Json};
use serde::Serialize;

/// This response can be returned from any handler when the request could not be completed, for
/// any reason.
#[derive(Serialize)]
pub struct ErrorResponse {
    /// A user friendly error message.
    /// These may or may not be displayed frontend, and should not contain sensitive information.
    pub message: String,
}

impl ErrorResponse {
    /// Creates a new `ErrorResponse` with the given message.
    pub fn new(message: &str) -> Self {
        ErrorResponse {
            message: String::from(message),
        }
    }

    /// The response returned when a token authentication fails.
    pub fn token_auth_failure() -> Self {
        ErrorResponse::new("Token authentication failed.")
    }

    /// The response returned when a token could not be created.
    /// **This should never really happen!**
    pub fn token_creation_failure() -> Self {
        ErrorResponse::new("Error creating authentication token.")
    }
}

/// This enum should be returned by all handlers in this module.
#[derive(Serialize)]
#[serde(untagged)]
pub enum ResponseBody<T: Serialize> {
    Pass(T),
    Fail(ErrorResponse),
}

/// This is returned from both the registration request and the sign in request.
#[derive(Serialize)]
pub struct TokenResponse {
    pub token: String,
}
