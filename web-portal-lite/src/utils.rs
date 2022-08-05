use crate::config::ServerConfig;
use crate::config::{
    UserConfig, UserConfigAccount, UserConfigBase, UserConfigDashboard, CURRENT_USER_CONFIG_VER,
};
use rocket::fs::NamedFile;
use rocket::http::CookieJar;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Outcome, Request};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
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
pub fn read_user_config(config_path: &PathBuf) -> Result<UserConfig, UserConfigError> {
    // TODO convert to async using tokio
    // read file
    let file = match File::open(config_path) {
        Ok(file_obj) => file_obj,
        Err(_) => return Err(UserConfigError::FileAccessError),
    };
    // parse file
    let user_config: UserConfig = match serde_yaml::from_reader(BufReader::new(file)) {
        Ok(loaded_config) => loaded_config,
        Err(_) => return Err(UserConfigError::ParseError),
    };
    // config version number check
    if user_config.config_version != CURRENT_USER_CONFIG_VER {
        return Err(UserConfigError::VersionNotSupported);
    }
    Ok(user_config)
}

/// Checks whether the user config file is supported,
/// by checking the version number
pub fn is_user_config_supported(config_path: &PathBuf) -> Result<(), UserConfigError> {
    // TODO convert to async using tokio
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

/// Returns the user's dashboard or default if none can be found
pub fn get_user_dashboard_or_default<'conf>(
    user_config: &'conf UserConfig,
    default_dashboard: &'conf Vec<UserConfigDashboard>,
    username: &str,
) -> &'conf Vec<UserConfigDashboard> {
    match user_config.accounts.get(username) {
        Some(account) => match account.dashboard.as_ref() {
            Some(dash) => dash,
            None => default_dashboard,
        },
        None => default_dashboard,
    }
}

/// Authenticated user for the request
pub struct User {
    pub username: String,
    pub is_public_acc: bool,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, ()> {
        let server_config = req
            .rocket()
            .state::<ServerConfig>()
            .map(|server_config| server_config);
        let user_config = req
            .rocket()
            .state::<UserConfig>()
            .map(|user_config| user_config);

        match (server_config, user_config) {
            (Some(s_config), Some(u_config)) => {
                match ensure_authenticated(req.cookies(), &u_config.accounts) {
                    Ok(username) => Outcome::Success(User {
                        username: username,
                        is_public_acc: false,
                    }),
                    Err(_) => match u_config.public_dash {
                        true => Outcome::Success(User {
                            username: s_config.public_dash_username.clone(),
                            is_public_acc: true,
                        }),
                        false => Outcome::Failure((Status::Unauthorized, ())),
                    },
                }
            }
            (_, _) => Outcome::Failure((Status::InternalServerError, ())),
        }
    }
}
