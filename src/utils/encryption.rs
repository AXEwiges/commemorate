use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

use crate::error::{CommemorateError, CommemorateResult};

pub const NONCE_LENGTH: usize = 12;
pub const SALT_LENGTH: usize = 22;

pub fn hash_password(password: &str) -> CommemorateResult<Vec<u8>> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| CommemorateError::EncryptionError(e.to_string()))?
        .to_string();
    Ok(password_hash.into_bytes())
}

pub fn verify_password(password: &str, password_hash: &str) -> CommemorateResult<bool> {
    let parsed_hash = PasswordHash::new(password_hash)
        .map_err(|e| CommemorateError::DecryptionError(e.to_string()))?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

pub fn encrypt(data: &[u8], password: &str) -> CommemorateResult<Vec<u8>> {
    let salt = SaltString::generate(&mut OsRng);

    let key = derive_key(password, salt.as_str())?;
    let cipher = Aes256Gcm::new(&key.into());
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = cipher
        .encrypt(&nonce, data)
        .map_err(|e| CommemorateError::EncryptionError(e.to_string()))?;
    let mut result = Vec::new();
    result.extend_from_slice(salt.as_str().as_bytes());
    result.extend_from_slice(&nonce);
    result.extend_from_slice(&ciphertext);
    Ok(result)
}

pub fn decrypt(encrypted_data: &[u8], password: &str) -> CommemorateResult<Vec<u8>> {
    if encrypted_data.len() < SALT_LENGTH + 1 {
        return Err(CommemorateError::DecryptionError(
            "Invalid encrypted data".to_string(),
        ));
    }

    let salt = std::str::from_utf8(&encrypted_data[..SALT_LENGTH])
        .map_err(|e| CommemorateError::DecryptionError(e.to_string()))?;
    let key = derive_key(password, salt)?;
    let cipher = Aes256Gcm::new(&key.into());

    let nonce_start = SALT_LENGTH;
    let nonce_end = nonce_start + 12;

    if encrypted_data.len() < nonce_end {
        return Err(CommemorateError::DecryptionError(
            "Invalid encrypted data".to_string(),
        ));
    }

    let nonce = Nonce::from_slice(&encrypted_data[nonce_start..nonce_end]);
    let plaintext = cipher
        .decrypt(nonce, &encrypted_data[nonce_end..])
        .map_err(|e| CommemorateError::DecryptionError(e.to_string()))?;
    Ok(plaintext)
}

fn derive_key(password: &str, salt: &str) -> CommemorateResult<[u8; 32]> {
    let argon2 = Argon2::default();
    let mut key = [0u8; 32];
    argon2
        .hash_password_into(password.as_bytes(), salt.as_bytes(), &mut key)
        .map_err(|e| CommemorateError::EncryptionError(e.to_string()))?;
    Ok(key)
}
