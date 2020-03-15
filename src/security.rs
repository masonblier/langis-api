use rand::Rng;
use argonautica::{Hasher, Verifier};

use crate::errors::ServiceError;

lazy_static::lazy_static! {
    /// loads SECRET_KEY environment variable for use in password hash validation
    pub static ref SECRET_KEY: String = std::env::var("SECRET_KEY")
        .expect("\n\n  Cowardly refusing to run without SECRET_KEY=(32-character string) environment variable\n\n");
}


/// length of secure session access tokens
const TOKEN_LENGTH: usize = 32;

/// Generates a random 32-character url-safe base64 token
pub fn random_token() -> Result<String, ServiceError> {
    const URL_BASE64_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789_-";
    let mut rng = rand::thread_rng();

    let token: String = (0..TOKEN_LENGTH)
        .map(|_| {
            let idx = rng.gen_range(0, URL_BASE64_CHARSET.len());
            URL_BASE64_CHARSET[idx] as char
        })
        .collect();

    Ok(token)
}

/// Hashes a text password to argon2-compatible hash
pub fn hash_password(password: &str) -> Result<String, ServiceError> {
    Hasher::default()
        .with_password(password)
        .with_secret_key(SECRET_KEY.as_str())
        .hash()
        .map_err(|err| {
            dbg!(err);
            ServiceError::InternalServerError
        })
}

/// Verifies a text password against an argon2-compatible hash
pub fn verify_password(hash: &str, password: &str) -> Result<bool, ServiceError> {
    Verifier::default()
        .with_hash(hash)
        .with_password(password)
        .with_secret_key(SECRET_KEY.as_str())
        .verify()
        .map_err(|err| {
            dbg!(err);
            ServiceError::Unauthorized
        })
}