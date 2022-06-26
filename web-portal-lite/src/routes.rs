use crate::config::{ServerConfig, UserConfig, UserConfigDashboard};
use crate::utils::{ensure_authenticated, get_user_dashboard_or_default, load_named_icon};
use rocket::form::{Form, FromForm, Strict};
use rocket::fs::NamedFile;
use rocket::http::{Cookie, CookieJar};
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket::{get, post, uri, State};
use rocket_dyn_templates::{context, Template};
use web_portal_lite_core::verify_hashed_password;

#[get("/")]
pub fn index(
    flash: Option<FlashMessage<'_>>,
    cookies: &CookieJar<'_>,
    server_config: &State<ServerConfig>,
    user_config: &State<UserConfig>,
) -> Result<Template, Flash<Redirect>> {
    let is_authenticated;
    let dashboard: &Vec<UserConfigDashboard>;
    let empty_dashboard = vec![];

    match ensure_authenticated(cookies, &user_config.accounts) {
        Ok(username) => {
            is_authenticated = true;
            dashboard = get_user_dashboard_or_default(user_config, &empty_dashboard, &username);
        }
        Err(_) => {
            // Check if login is required to access portal
            if user_config.public_dash == false {
                return Err(Flash::error(
                    Redirect::to(uri!(get_login)),
                    "login is required to access this portal",
                ));
            }
            is_authenticated = false;
            dashboard = get_user_dashboard_or_default(
                user_config,
                &empty_dashboard,
                &server_config.public_dash_username,
            );
        }
    };

    Ok(Template::render(
        "index",
        context!(
            flashed_message: flash,
            is_authenticated: is_authenticated,
            dashboard: dashboard
        ),
    ))
}

#[get("/icons/<icon_name>")]
pub async fn get_icon(icon_name: &str, config: &State<ServerConfig>) -> Option<NamedFile> {
    load_named_icon(icon_name, &config.icons_path).await
}

#[get("/auth/login")]
pub fn get_login(
    flash: Option<FlashMessage<'_>>,
    cookies: &CookieJar<'_>,
    user_config: &State<UserConfig>,
) -> Result<Template, Flash<Redirect>> {
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
    server_config: &State<ServerConfig>,
    user_config: &State<UserConfig>,
    login_form: Form<Strict<UserLoginForm>>,
) -> Result<Redirect, Flash<Redirect>> {
    // FIXME remove unwrap use here
    let username = login_form.username.to_string();

    // ensure username is not the public virtual account when public mode is on
    if username == server_config.public_dash_username && user_config.public_dash == true {
        return Err(Flash::error(
            Redirect::to(uri!(get_login)),
            "login to this account is not allowed",
        ));
    }

    match user_config.accounts.get(&username) {
        Some(user) => {
            if verify_hashed_password(&login_form.password, &user.password).unwrap() {
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
