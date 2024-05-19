use tower_cookies::{cookie::time::Duration, Cookie, Cookies};

use crate::api::defs::CookiesNames;

pub fn add_auth_token(cookies: &Cookies, token: String) {
    let mut cookie = Cookie::new(CookiesNames::AUTH_TOKEN, token);

    cookie.set_http_only(true);
    cookie.set_max_age(Duration::hours(1));

    cookies.add(cookie);
}
