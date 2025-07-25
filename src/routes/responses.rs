use serde::Serialize;

/// This response can be returned from any handler when the request could not be completed, for
/// any reason.
#[derive(Serialize)]
pub struct ErrorResponse {
    /// A user friendly error message.
    /// These may or may not be displayed frontend, and should not contain sensitive information.
    pub message: String,
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
