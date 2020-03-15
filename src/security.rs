use rand::Rng;
use argonautica::{Hasher, Verifier};

use crate::errors::ServiceError;

lazy_static::lazy_static! {
    pub static ref SECRET_KEY: String = std::env::var("SECRET_KEY")
        .expect("\n\n  Cowardly refusing to run without SECRET_KEY=(32-character string) environment variable\n\n");
}
const TOKEN_LENGTH: usize = 32;

/// Generates a random 32-character url-safe base64 token
pub fn random_token() -> Result<String, ServiceError> {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    let mut rng = rand::thread_rng();

    let token: String = (0..TOKEN_LENGTH)
        .map(|_| {
            let idx = rng.gen_range(0, CHARSET.len());
            CHARSET[idx] as char
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