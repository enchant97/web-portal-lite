use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

/// Returns a hashed password using Argon2 into the 'PHC string' format
pub fn create_hashed_password(password: &str) -> Result<String, ()> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    match argon2.hash_password(password.as_bytes(), &salt) {
        Ok(hash) => Ok(hash.to_string()),
        Err(_) => Err(()),
    }
}

/// Verifies hashed password in the 'PHC string' format with a plain text one
pub fn verify_hashed_password(password: &str, password_hash: &str) -> Result<bool, ()> {
    match PasswordHash::new(password_hash) {
        Ok(hash) => Ok(Argon2::default()
            .verify_password(password.as_bytes(), &hash)
            .is_ok()),
        Err(_) => Err(()),
    }
}

/// Check whether password has been hashed
pub fn is_hashed_password(possible_hash: &str) -> bool {
    match PasswordHash::new(possible_hash) {
        Ok(_) => true,
        Err(_) => false,
    }
}
