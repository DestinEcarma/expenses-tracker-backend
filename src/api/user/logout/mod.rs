use axum::{http::StatusCode, response::IntoResponse};
use tower_cookies::{cookie::time::Duration, Cookie, Cookies};

use crate::api::defs::CookieNames;

pub async fn handler(cookies: Cookies) -> impl IntoResponse {
    let mut cookie = Cookie::new(CookieNames::AUTH_TOKEN, "");

    cookie.set_http_only(true);
    cookie.set_path("/");
    cookie.set_max_age(Duration::ZERO);

    cookies.add(cookie);

    (StatusCode::OK).into_response()
}
