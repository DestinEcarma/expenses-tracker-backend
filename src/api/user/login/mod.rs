mod utils;

use self::utils::{get_user, verify_password};
use super::add_auth_token;
use crate::{api::db::defs::ExtensionDB, error::Result};
use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use tower_cookies::Cookies;

#[derive(Debug, Deserialize)]
pub struct LoginPayLoad {
    pub username: String,
    pub password: String,
}

pub async fn handler(
    cookies: Cookies,
    db: ExtensionDB,
    payload: Json<LoginPayLoad>,
) -> Result<impl IntoResponse> {
    let user_login = get_user(&db, &payload.username).await?;

    verify_password(&payload.password, user_login.password()).await?;
    add_auth_token(&cookies, user_login.id().id.to_raw());

    Ok(StatusCode::CREATED)
}
