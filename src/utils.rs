use crate::config::{UserConfig, UserConfigAccount};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rocket::fs::NamedFile;
use rocket::http::CookieJar;
use serde_yaml;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};

/// Will try to load icon from svg folder then png.
pub async fn load_named_icon(icon_name: &str, icons_path: &PathBuf) -> Option<NamedFile> {
    let svg_path = Path::new(icons_path)
        .join("svg")
        .join(String::from(icon_name) + ".svg");
    let png_path = Path::new(icons_path)
        .join("png")
        .join(String::from(icon_name) + ".png");

    let svg_file = NamedFile::open(svg_path).await.ok();

    match svg_file {
        Some(_) => svg_file,
        None => NamedFile::open(png_path).await.ok(),
    }
}

/// Read a user config file and return it
pub fn read_user_config(config_path: &PathBuf) -> std::io::Result<UserConfig> {
    // TODO convert to async using tokio
    // FIXME get rid of unwrap usage
    let file = File::open(config_path)?;
    let user_config: UserConfig = serde_yaml::from_reader(BufReader::new(file)).unwrap();
    Ok(user_config)
}

pub fn write_user_config(config_path: &PathBuf, config: &UserConfig) {
    // TODO convert to async using tokio
    // FIXME get rid of unwrap usage
    let file = File::create(config_path).unwrap();
    serde_yaml::to_writer(BufWriter::new(file), config).unwrap();
}

/// Returns whether user is authenticated and actually exists
pub fn ensure_authenticated(
    cookies: &CookieJar,
    accounts: &HashMap<String, UserConfigAccount>,
) -> Result<(), ()> {
    match cookies.get_private("AUTH") {
        Some(cookie) => match accounts.get(cookie.value()) {
            Some(_) => Ok(()),
            None => Err(()),
        },
        None => Err(()),
    }
}

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
