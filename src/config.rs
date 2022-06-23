use rocket::serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub config_path: PathBuf,
    pub icons_path: PathBuf,
    pub guest_allowed: bool,
}
