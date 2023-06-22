use crate::config::ServerConfig;
use crate::config::{UserConfig, UserConfigAccount, UserConfigDashboard};
use rocket::fs::NamedFile;
use rocket::http::CookieJar;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Outcome, Request};
use std::collections::HashMap;
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

/// Returns whether user is authenticated and actually exists
pub fn ensure_authenticated(
    cookies: &CookieJar,
    accounts: &HashMap<String, UserConfigAccount>,
) -> Result<String, ()> {
    match cookies.get_private("AUTH") {
        Some(cookie) => match accounts.get(cookie.value()) {
            Some(_) => Ok(cookie.value().to_owned()),
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
        let server_config = req.rocket().state::<ServerConfig>();
        let user_config = req.rocket().state::<UserConfig>();

        match (server_config, user_config) {
            (Some(s_config), Some(u_config)) => {
                match ensure_authenticated(req.cookies(), &u_config.accounts) {
                    Ok(username) => Outcome::Success(User {
                        username,
                        is_public_acc: false,
                    }),
                    Err(_) => match u_config.public_dash {
                        true => Outcome::Success(User {
                            username: s_config.public_dash_username.to_owned(),
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
