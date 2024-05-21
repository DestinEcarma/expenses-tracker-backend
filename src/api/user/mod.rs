mod auth;
mod login;
mod logout;
mod sign_up;

use crate::api::defs::CookieNames;
use axum::{routing, Router};
use tower_cookies::{cookie::time::Duration, Cookie, Cookies};

pub fn router() -> Router {
    Router::new()
        .route("/login", routing::post(login::handler))
        .route("/sign-up", routing::post(sign_up::handler))
        .route("/logout", routing::delete(logout::handler))
        .nest("/auth", auth::router())
}

fn add_auth_token(cookies: &Cookies, token: String) {
    let mut cookie = Cookie::new(CookieNames::AUTH_TOKEN, token);

    cookie.set_http_only(true);
    cookie.set_path("/");
    cookie.set_max_age(Duration::days(30));

    cookies.add(cookie);
}
