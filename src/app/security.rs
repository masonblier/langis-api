use rand::Rng;
use argonautica::{Hasher, Verifier};

use crate::app::config;
use crate::app::errors::ServiceError;

/// Generates a random 32-character url-safe base64 token
pub fn random_token() -> Result<String, ServiceError> {
    const URL_BASE64_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789_-";
    let mut rng = rand::thread_rng();

    let token: String = (0..config::TOKEN_LENGTH)
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
        .with_secret_key(config::SECRET_KEY.as_str())
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
        .with_secret_key(config::SECRET_KEY.as_str())
        .verify()
        .map_err(|err| {
            dbg!(err);
            ServiceError::Unauthorized
        })
}
