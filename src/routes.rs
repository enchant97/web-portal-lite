use crate::config::ServerConfig;
use crate::utils::{ensure_authenticated, load_named_icon, read_user_config};
use rocket::form::{Form, FromForm, Strict};
use rocket::fs::NamedFile;
use rocket::http::{Cookie, CookieJar};
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket::{get, post, uri, State};
use rocket_dyn_templates::{context, Template};

#[get("/")]
pub fn index(flash: Option<FlashMessage<'_>>) -> Template {
    Template::render("index", context!(flashed_message: flash))
}

#[get("/icons/<icon_name>")]
pub async fn get_icon(icon_name: &str, config: &State<ServerConfig>) -> Option<NamedFile> {
    load_named_icon(icon_name, &config.icons_path).await
}

#[get("/auth/login")]
pub fn get_login(
    flash: Option<FlashMessage<'_>>,
    cookies: &CookieJar<'_>,
    config: &State<ServerConfig>,
) -> Result<Template, Flash<Redirect>> {
    // FIXME remove unwrap use here
    let user_config = read_user_config(&config.config_path).unwrap();

    match ensure_authenticated(cookies, &user_config.accounts).is_ok() {
        true => Err(Flash::warning(
            Redirect::to(uri!(index)),
            "you can't login while already logged in",
        )),
        false => Ok(Template::render("login", context!(flashed_message: flash))),
    }
}

#[derive(FromForm)]
pub struct UserLoginForm {
    username: String,
    password: String,
}

#[post("/auth/login", data = "<login_form>")]
pub fn post_login(
    cookies: &CookieJar<'_>,
    config: &State<ServerConfig>,
    login_form: Form<Strict<UserLoginForm>>,
) -> Result<Redirect, Flash<Redirect>> {
    // FIXME remove unwrap use here
    let user_config = read_user_config(&config.config_path).unwrap();

    let username = login_form.username.to_string();

    match user_config.accounts.get(&username) {
        Some(user) => {
            if user.password == login_form.password {
                cookies.add_private(Cookie::new("AUTH", username));
                return Ok(Redirect::to(uri!(index)));
            }
            return Err(Flash::error(
                Redirect::to(uri!(get_login)),
                "username or password invalid",
            ));
        }
        None => Err(Flash::error(
            Redirect::to(uri!(get_login)),
            "username or password invalid",
        )),
    }
}

#[get("/auth/logout")]
pub fn get_logout(cookies: &CookieJar<'_>) -> Flash<Redirect> {
    cookies.remove_private(Cookie::named("AUTH"));
    Flash::success(Redirect::to(uri!(index)), "you have been logged out")
}
