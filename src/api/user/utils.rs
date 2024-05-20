use tower_cookies::{cookie::time::Duration, Cookie, Cookies};

use crate::api::defs::CookieNames;

pub fn add_auth_token(cookies: &Cookies, token: String) {
    let mut cookie = Cookie::new(CookieNames::AUTH_TOKEN, token);

    cookie.set_http_only(true);
    cookie.set_path("/");
    cookie.set_max_age(Duration::hours(1));

    cookies.add(cookie);
}
