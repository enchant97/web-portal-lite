use rocket::{launch, routes};
use rocket_dyn_templates::Template;

mod routes;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![routes::index])
}
