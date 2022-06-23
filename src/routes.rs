use crate::config::Config;
use crate::utils::load_named_icon;
use rocket::fs::NamedFile;
use rocket::{get, State};
use rocket_dyn_templates::{context, Template};

#[get("/")]
pub fn index() -> Template {
    Template::render("index", context!())
}

#[get("/icons/<icon_name>")]
pub async fn get_icon(icon_name: &str, config: &State<Config>) -> Option<NamedFile> {
    load_named_icon(icon_name, &config.icons_path).await
}
