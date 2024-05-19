use axum::{http::StatusCode, response::IntoResponse};
use tower_cookies::{Cookie, Cookies};

use crate::api::defs::CookiesNames;

pub async fn handler(cookies: Cookies) -> impl IntoResponse {
    cookies.remove(Cookie::from(CookiesNames::AUTH_TOKEN));

    (StatusCode::OK).into_response()
}
