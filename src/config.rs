use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub config_path: PathBuf,
    pub icons_path: PathBuf,
    pub guest_allowed: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserConfigLink {
    title: String,
    color_name: Option<String>,
    icon_name: Option<String>,
    href: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserConfigDashboard {
    title: String,
    show_header: bool,
    links: Vec<UserConfigLink>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserConfigAccount {
    password: String,
    dashboard: Option<Vec<UserConfigDashboard>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserConfig {
    config_version: usize,
    accounts: HashMap<String, UserConfigAccount>,
}
