use rocket::fairing::AdHoc;
use rocket::fs::{relative, FileServer};
use rocket::{launch, routes};
use rocket_dyn_templates::Template;

mod config;
mod routes;
mod utils;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![
                routes::index,
            ]
        )
        .mount("/static", routes!(routes::get_icon))
        .mount("/static", FileServer::from(relative!("static/")))
        .attach(AdHoc::config::<config::ServerConfig>())
}
