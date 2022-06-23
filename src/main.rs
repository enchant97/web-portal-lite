use rocket::fairing::AdHoc;
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
                routes::get_icon,
            ]
        )
        .attach(AdHoc::config::<config::Config>())
}
