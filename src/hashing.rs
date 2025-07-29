//! This module provides functions for securely hashing passwords
//! and verifying raw passwords against stored Argon2 hashes.
//!
//! # Features
//!
//! - Uses the Argon2 password hashing algorithm via the `argon2` crate.
//! - Automatically generates a secure random salt for each password.
//! - Verifies passwords against PHC-formatted hashes.
//!
//! # Example
//!
//! ```rust
//! use crate::hashing::{hash_password, verify_password};
//!
//! let password = "hunter2";
//!
//! let hash = hash_password(password).expect("Hashing failed");
//! assert!(verify_password(password, &hash).unwrap());
//! ```
//!
//! # Notes
//!
//! - All returned hashes are in [PHC string format](https://github.com/P-H-C/phc-string-format).
//! - The hashing algorithm is Argon2id with default parameters.
//! - Do not compare hashes manually—always use `verify_password`.

use argon2::{
    password_hash::{
        rand_core::OsRng, Error as HashError, PasswordHash, PasswordHasher, SaltString,
    },
    Argon2, PasswordVerifier,
};

/// Hashes a raw password using Argon2 and a randomly generated salt.
///
/// # Arguments
///
/// * `password` - The raw plaintext password to be hashed.
///
/// # Returns
///
/// * `Ok(String)` containing the password hash in PHC string format.
/// * `Err(HashError)` if hashing fails.
pub fn hash_password(password: &str) -> Result<String, HashError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    Ok(argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string())
}

/// Verify that a raw password matches the provided hashed password.
///
/// # Arguments
///
/// * `password` - The raw plaintext password provided by the user.
/// * `hash` - The previously computed Argon2 password hash (in PHC string format).
///
/// # Returns
///
/// * `Ok(true)` if the password matches the hash.
/// * `Ok(false)` if the password does not match.
///
/// # Errors
///
/// Returns a `HashError` when the provided hash cannot be parsed.
pub fn verify_password(password: &str, hash: &str) -> Result<bool, HashError> {
    let parsed_hash = PasswordHash::new(hash)?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_hashing() {
        let password = "hunter2";
        let hash = hash_password(password).expect("Hashing failed");
        assert!(verify_password(password, &hash).unwrap());
        assert!(!verify_password("wrongpassword", &hash).unwrap());
    }
}
