use crate::config::{UserConfig, UserConfigAccount, UserConfigBase, CURRENT_USER_CONFIG_VER};
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

#[derive(Debug)]
pub enum UserConfigError {
    FileAccessError,
    ParseError,
    VersionNotSupported,
}

/// Read a user config file and return it
pub fn read_user_config(config_path: &PathBuf) -> std::io::Result<UserConfig> {
    // TODO convert to async using tokio
    // FIXME get rid of unwrap usage
    let file = File::open(config_path)?;
    let user_config: UserConfig = serde_yaml::from_reader(BufReader::new(file)).unwrap();
    Ok(user_config)
}

/// Checks whether the user config file is supported,
/// by checking the version number
pub fn is_user_config_supported(config_path: &PathBuf) -> Result<(), UserConfigError> {
    // open file
    let file = match File::open(config_path) {
        Ok(file_obj) => file_obj,
        Err(_) => return Err(UserConfigError::FileAccessError),
    };
    // parse file
    let user_config: UserConfigBase = match serde_yaml::from_reader(BufReader::new(file)) {
        Ok(loaded_config) => loaded_config,
        Err(_) => return Err(UserConfigError::ParseError),
    };
    // version check
    match user_config.config_version {
        CURRENT_USER_CONFIG_VER => Ok(()),
        _ => Err(UserConfigError::VersionNotSupported),
    }
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
) -> Result<String, ()> {
    match cookies.get_private("AUTH") {
        Some(cookie) => match accounts.get(cookie.value()) {
            Some(_) => Ok(cookie.value().to_string()),
            None => Err(()),
        },
        None => Err(()),
    }
}
