use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

pub const CURRENT_USER_CONFIG_VER: usize = 1;

#[derive(Debug)]
pub enum UserConfigError {
    FileAccessError,
    ParseError,
    VersionNotSupported,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub config_path: PathBuf,
    pub icons_path: PathBuf,
    pub public_dash_username: String,
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
    pub compact: bool,
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

impl UserConfig {
    /// Create from reading a yaml file
    pub fn from_yaml_file(config_path: &PathBuf) -> Result<Self, UserConfigError> {
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

    pub fn create_template() -> Self {
        let mut accounts = HashMap::new();
        accounts.insert(
            "public".to_owned(),
            UserConfigAccount {
                password: "".to_owned(),
                dashboard: Some(vec![UserConfigDashboard {
                    title: "Group One".to_owned(),
                    compact: false,
                    show_header: true,
                    links: vec![UserConfigLink {
                        title: "Link One".to_owned(),
                        color_name: "no-color".to_owned(),
                        icon_name: "".to_owned(),
                        href: "".to_owned(),
                    }],
                }]),
            },
        );
        UserConfig {
            config_version: CURRENT_USER_CONFIG_VER,
            public_dash: true,
            accounts,
        }
    }

    /// Check whether any accounts are available (excluding public virtual)
    pub fn has_users(&self, public_username: &String) -> bool {
        if self.accounts.is_empty()
            || (self.accounts.len() == 1 && self.accounts.contains_key(public_username))
        {
            return false;
        }
        true
    }

    /// Serialise into a yaml string
    pub fn to_yaml_string(&self) -> String {
        serde_yaml::to_string(self).expect("failed to serialise UserConfig to yaml string")
    }
}
