use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher, PasswordVerifier};

use crate::configuration::Error;

/// Checks if the provided password matches the given hash.
/// Returns true on success.
pub async fn validate_password(password: String, hash: String) -> Result<bool, Error> {
    tokio::task::spawn_blocking(move || -> Result<bool, Error> {
        let hash = argon2::password_hash::PasswordHash::new(hash.as_str()).map_err(|e| {
            tracing::warn!("Error while hashing password: {}", e);

            Error::Internal
        })?;

        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &hash)
            .is_ok())
    })
    .await
    .map_err(|e| {
        tracing::warn!("Error spawning hashing task: {:?}", e);

        Error::Internal
    })?
}

pub async fn hash_password(password: String) -> Result<String, Error> {
    let argon = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);

    tokio::task::spawn_blocking(move || -> Result<String, Error> {
        let hash = argon
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| {
                tracing::warn!("Error while hashing password: {}", e);

                Error::Internal
            })?;

        Ok(hash.to_string())
    })
    .await
    .map_err(|e| {
        tracing::warn!("Error spawning hashing task: {:?}", e);

        Error::Internal
    })?
}
