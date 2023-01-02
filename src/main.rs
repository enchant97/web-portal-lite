use clap::Parser;
use config::{UserConfig, UserConfigError};
use rocket::fairing::AdHoc;
use rocket::fs::{relative, FileServer};
use rocket::{catchers, routes};
use rocket_dyn_templates::Template;
use std::io;
use std::io::Write;

mod args;
mod config;
mod password;
mod routes;
mod utils;

use crate::password::create_hashed_password;

/// The current app version
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn handle_serve() -> rocket::Rocket<rocket::Build> {
    let mut rocket = rocket::build()
        .attach(Template::fairing())
        .mount(
            "/",
            routes![
                routes::index,
                routes::get_login,
                routes::post_login,
                routes::get_logout,
            ],
        )
        .register("/", catchers![routes::catch_unauthorized])
        .mount("/static", routes!(routes::get_icon))
        .mount("/static", FileServer::from(relative!("static/")))
        .attach(AdHoc::config::<config::ServerConfig>());

    let server_config: config::ServerConfig = rocket.figment().extract().expect("config");

    // early full user config parse, so routes can use it as a State
    let user_config = match UserConfig::from_yaml_file(&server_config.config_path) {
        Ok(loaded_config) => loaded_config,
        Err(error) => match error {
            UserConfigError::FileAccessError => panic!("user config file access error"),
            UserConfigError::ParseError => {
                panic!("user config parse error, does config match version number?")
            }
            UserConfigError::VersionNotSupported => {
                panic!("user config version not supported")
            }
        },
    };
    // NOTE a Mutex might need to be added to allow state to be modified
    rocket = rocket.manage(user_config);

    rocket
}

fn handle_password_hasher() {
    let mut password = String::new();
    print!("enter password: ");
    io::stdout().flush().unwrap();
    match io::stdin().read_line(&mut password) {
        Ok(_) => match create_hashed_password(password.trim()) {
            Some(hashed_pw) => println!("hashed password: {hashed_pw}"),
            None => eprintln!("error hashing password"),
        },
        Err(error) => eprintln!("error: {error}"),
    }
}

fn handle_generate_config() {
    let config = UserConfig::create_template().to_yaml_string();
    print!("{}", config);
}

fn handle_version() {
    println!("v{}", VERSION)
}

#[rocket::main]
async fn main() {
    let args = args::Args::parse();
    match args.cmd {
        args::Command::Serve => {
            let _ = handle_serve().launch().await;
        }
        args::Command::PwHasher => handle_password_hasher(),
        args::Command::GenConfig => handle_generate_config(),
        args::Command::Version => handle_version(),
    };
}
