use argon2::{
    password_hash::{
        rand_core::OsRng, Error as HashError, PasswordHash, PasswordHasher, SaltString,
    },
    Argon2, PasswordVerifier,
};

/// Convert a raw text password to a safe hash.
pub fn hash_password(password: &str) -> Result<String, HashError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash: Result<argon2::PasswordHash<'_>, argon2::password_hash::Error> =
        argon2.hash_password(password.as_bytes(), &salt);
    match password_hash {
        Ok(hash) => Ok(hash.to_string()),
        Err(err) => Err(err),
    }
}

/// Verify that password is the unhashed version of hash.
pub fn verify_password(password: &str, hash: &str) -> Result<(), HashError> {
    let parsed_hash = PasswordHash::new(hash);
    let parsed_hash = match parsed_hash {
        Ok(ph) => ph,
        Err(err) => return Err(err),
    };
    Argon2::default().verify_password(password.as_bytes(), &parsed_hash)
}
