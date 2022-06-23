use rocket::fairing::AdHoc;
use rocket::{launch, routes};
use rocket_dyn_templates::Template;

mod config;
mod routes;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![routes::index])
        .attach(AdHoc::config::<config::Config>())
}
