use std::env;

use jsonwebtoken::{
    decode, encode, errors::Error as JWTError, get_current_timestamp, Algorithm, DecodingKey,
    EncodingKey, Header, TokenData, Validation,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// The unique inputs required for creating a new authentication token.
pub struct AuthnTokenReqs {
    pub id: Uuid,
    pub username: String,
    pub email: String,
}

/// The full payload of an authentication token.
#[derive(Serialize, Deserialize)]
pub struct AuthnTokenPayload {
    pub sub: Uuid,
    pub username: String,
    pub email: String,
    pub iat: u64,
    pub exp: u64,
    pub iss: String,
    pub nbf: u64,
}

impl AuthnTokenPayload {
    /// Create a new AuthnTokenPayload which is valid for 60 minutes following
    /// its creation.
    ///
    /// # Arguments
    ///
    /// * `reqs` - The unique inputs to be encoded in this token.
    fn new(reqs: AuthnTokenReqs) -> Self {
        let iat = get_current_timestamp();
        Self {
            sub: reqs.id,
            username: reqs.username,
            email: reqs.email,
            iat: iat,
            nbf: iat,
            exp: iat + 3600,
            iss: String::from("bitcasino.bigdevdog.com"),
        }
    }
}

/// Encodes a new authentication token valid for 60 minutes.
///
/// # Arguments
///
/// * `reqs` - The unique inputs to be encoded into the token.
///
/// # Returns
///
/// * `Ok(String)` when the token generates successfully.
/// * `Err(JWTError)` when the token cannot be encoded.
pub fn encode_authn_token(reqs: AuthnTokenReqs) -> Result<String, JWTError> {
    let payload = AuthnTokenPayload::new(reqs);
    let secret =
        env::var("JWT_SECRET").expect("Environment is not set up properly; missing 'JWT_SECRET'.");
    encode(
        &Header::default(),
        &payload,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

/// Decodes an authentication token.
///
/// # Arguments
///
/// * `token` - The JWT to decode.
///
/// # Returns
///
/// * `Ok(TokenData<AuthnTokenPayload>)` when the token is decoded.
/// * `Err(JWTError)` if the token cannot be decoded.
pub fn decode_authn_token(token: String) -> Result<TokenData<AuthnTokenPayload>, JWTError> {
    decode(
        &token,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::new(Algorithm::HS256),
    )
}
