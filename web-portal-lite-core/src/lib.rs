use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

/// The current app version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Returns a hashed password using Argon2 into the 'PHC string' format
pub fn create_hashed_password(password: &str) -> Option<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    match argon2.hash_password(password.as_bytes(), &salt) {
        Ok(hash) => Some(hash.to_string()),
        Err(_) => None,
    }
}

/// Verifies hashed password in the 'PHC string' format with a plain text one
pub fn verify_hashed_password(password: &str, password_hash: &str) -> bool {
    match PasswordHash::new(password_hash) {
        Ok(hash) => Argon2::default()
            .verify_password(password.as_bytes(), &hash)
            .is_ok(),
        Err(_) => false,
    }
}

/// Check whether password has been hashed
pub fn is_hashed_password(possible_hash: &str) -> bool {
    PasswordHash::new(possible_hash).is_ok()
}
