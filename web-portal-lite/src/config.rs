use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

pub const CURRENT_USER_CONFIG_VER: usize = 1;

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub config_path: PathBuf,
    pub icons_path: PathBuf,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserConfigLink {
    pub title: String,
    pub color_name: String,
    pub icon_name: String,
    pub href: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserConfigDashboard {
    pub title: String,
    pub show_header: bool,
    pub links: Vec<UserConfigLink>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserConfigAccount {
    pub password: String,
    pub dashboard: Option<Vec<UserConfigDashboard>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserConfigBase {
    pub config_version: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserConfig {
    pub config_version: usize,
    pub public_dash: bool,
    pub accounts: HashMap<String, UserConfigAccount>,
}
