// this hides warning when Flash<Redirect> is used (issue with rocket?)
#![allow(clippy::result_large_err)]
use crate::config::{ServerConfig, UserConfig, UserConfigDashboard};
use crate::password::verify_hashed_password;
use crate::utils::{ensure_authenticated, get_user_dashboard_or_default, load_named_icon, User};
use rocket::form::{Form, FromForm, Strict};
use rocket::fs::NamedFile;
use rocket::http::{Cookie, CookieJar};
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket::{catch, get, post, uri, State};
use rocket_dyn_templates::{context, Template};

pub type FlashedRedirect = Flash<Redirect>;

#[get("/")]
pub fn index(
    flash: Option<FlashMessage<'_>>,
    server_config: &State<ServerConfig>,
    user_config: &State<UserConfig>,
    user: User,
) -> Result<Template, FlashedRedirect> {
    let empty_dashboard = vec![];
    let is_authenticated = !user.is_public_acc;
    let dashboard: &Vec<UserConfigDashboard> = match is_authenticated {
        true => get_user_dashboard_or_default(user_config, &empty_dashboard, &user.username),
        false => get_user_dashboard_or_default(
            user_config,
            &empty_dashboard,
            &server_config.public_dash_username,
        ),
    };

    Ok(Template::render(
        "index",
        context!(
            flashed_message: flash,
            has_users: user_config.has_users(&server_config.public_dash_username),
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
    server_config: &State<ServerConfig>,
    user_config: &State<UserConfig>,
) -> Result<Template, FlashedRedirect> {
    match ensure_authenticated(cookies, &user_config.accounts).is_ok() {
        true => Err(Flash::warning(
            Redirect::to(uri!(index)),
            "you can't login while already logged in",
        )),
        false => Ok(Template::render(
            "login",
            context!(
                has_users: user_config.has_users(&server_config.public_dash_username),
                flashed_message: flash,
            ),
        )),
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
) -> Result<Redirect, FlashedRedirect> {
    let username = &login_form.username;

    // ensure username is not the public virtual account when public mode is on
    if username == &server_config.public_dash_username && user_config.public_dash {
        return Err(Flash::error(
            Redirect::to(uri!(get_login)),
            "login to this account is not allowed",
        ));
    }

    match user_config.accounts.get(username) {
        Some(user) => {
            if verify_hashed_password(&login_form.password, &user.password) {
                cookies.add_private(Cookie::new("AUTH", username.to_owned()));
                return Ok(Redirect::to(uri!(index)));
            }
            Err(Flash::error(
                Redirect::to(uri!(get_login)),
                "username or password invalid",
            ))
        }
        None => Err(Flash::error(
            Redirect::to(uri!(get_login)),
            "username or password invalid",
        )),
    }
}

#[get("/auth/logout")]
pub fn get_logout(cookies: &CookieJar<'_>, user_config: &State<UserConfig>) -> FlashedRedirect {
    cookies.remove_private(Cookie::named("AUTH"));

    match user_config.public_dash {
        true => Flash::success(Redirect::to(uri!(index)), "you have been logged out"),
        false => Flash::success(Redirect::to(uri!(get_login)), "you have been logged out"),
    }
}

#[catch(401)]
pub fn catch_unauthorized() -> FlashedRedirect {
    Flash::error(
        Redirect::to(uri!(get_login)),
        "login is required to access this portal",
    )
}
