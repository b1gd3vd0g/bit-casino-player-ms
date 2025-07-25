use serde::Serialize;

/// This is returned from both the registration request and the sign in request.
#[derive(Serialize)]
pub struct TokenResponse {
    pub token: String,
}

impl TokenResponse {
    pub fn new(token: String) -> Self {
        TokenResponse { token: token }
    }
}

#[derive(Serialize)]
pub struct MessageResponse {
    pub message: String,
}

impl MessageResponse {
    pub fn new(message: &str) -> Self {
        MessageResponse {
            message: String::from(message),
        }
    }

    pub fn token_auth_failure() -> Self {
        MessageResponse::new("Token authentication failed.")
    }

    pub fn token_creation_failure() -> Self {
        MessageResponse::new("Error creating authentication token.")
    }
}
