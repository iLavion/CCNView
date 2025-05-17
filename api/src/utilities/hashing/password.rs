// src/utilities/hashing/pwd.rs

use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::SaltString;
use rand::RngCore;
use thiserror::Error;
use log::debug;

#[derive(Debug, Error)]
pub enum PasswordError {
    #[error("Password hashing failed: {0}")]
    HashingFailed(argon2::password_hash::Error),

    #[error("Password verification failed: {0}")]
    VerificationFailed(argon2::password_hash::Error),
}

pub fn hash_password(password: &[u8]) -> Result<(String, String), PasswordError> {
  let mut salt = [0u8; 16]; // 16 bytes salt
  let mut rng = rand::rng();
  rng.fill_bytes(&mut salt);

  let argon2 = Argon2::default();
  let salt_string = SaltString::encode_b64(&salt).map_err(PasswordError::HashingFailed)?;
  let password_hash = argon2
      .hash_password(password, &salt_string)
      .map_err(PasswordError::HashingFailed)?;
  
  // Return password_hash first, then salt
  debug!("Full password hash: {}", password_hash.to_string());
  Ok((password_hash.to_string(), salt_string.to_string()))
}

pub fn verify_password(password: &[u8], _salt_encoded: &str, hash: &str) -> Result<(), PasswordError> {
    let argon2 = Argon2::default();
    let parsed_hash = PasswordHash::new(hash).map_err(PasswordError::VerificationFailed)?;
    argon2
        .verify_password(password, &parsed_hash)
        .map_err(PasswordError::VerificationFailed)?;
    Ok(())
}