use rocket::fairing::AdHoc;
use rocket::fs::{relative, FileServer};
use rocket::{launch, routes};
use rocket_dyn_templates::Template;

mod config;
mod routes;
mod utils;

#[launch]
fn rocket() -> _ {
    let rocket = rocket::build()
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
        .mount("/static", routes!(routes::get_icon))
        .mount("/static", FileServer::from(relative!("static/")))
        .attach(AdHoc::config::<config::ServerConfig>());

    let server_config: config::ServerConfig = rocket.figment().extract().expect("config");

    // ensure user config is valid before launching
    match utils::is_user_config_supported(&server_config.config_path) {
        Ok(_) => (),
        Err(error) => {
            match error {
                utils::UserConfigError::FileAccessError => panic!("user config file access error"),
                utils::UserConfigError::ParseError => panic!("user config parse error"),
                utils::UserConfigError::VersionNotSupported => {
                    panic!("user config version not supported")
                }
            };
        }
    };

    rocket
}
