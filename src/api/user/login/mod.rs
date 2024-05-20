mod defs;
mod utils;

use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use tower_cookies::Cookies;

use crate::api::db::defs::ExtensionDB;

use self::{
    defs::LoginPayLoad,
    utils::{get_user, verify_password},
};

use super::utils::add_auth_token;

pub async fn handler(
    cookies: Cookies,
    db: ExtensionDB,
    payload: Json<LoginPayLoad>,
) -> impl IntoResponse {
    let user_login = match get_user(&db, &payload.username).await {
        Err(error) => return error.into_response(),
        Ok(user) => user,
    };

    if let Err(error) = verify_password(&payload.password, user_login.password()).await {
        return error.into_response();
    }

    add_auth_token(&cookies, user_login.id().id.to_raw());

    (StatusCode::CREATED).into_response()
}
