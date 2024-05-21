use crate::{api::defs::CookieNames, error::Result};
use axum::{http::StatusCode, response::IntoResponse};
use tower_cookies::{cookie::time::Duration, Cookie, Cookies};

pub async fn handler(cookies: Cookies) -> Result<impl IntoResponse> {
    let mut cookie = Cookie::new(CookieNames::AUTH_TOKEN, "");

    cookie.set_http_only(true);
    cookie.set_path("/");
    cookie.set_max_age(Duration::ZERO);

    cookies.add(cookie);

    Ok(StatusCode::OK)
}
