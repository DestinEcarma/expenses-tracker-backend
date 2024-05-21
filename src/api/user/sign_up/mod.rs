mod utils;

use self::utils::{add_user, username_exists, validate_payload};
use super::add_auth_token;
use crate::{api::db::defs::ExtensionDB, error::Result};
use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use bcrypt::{hash, DEFAULT_COST};
use serde::Deserialize;
use tower_cookies::Cookies;

#[derive(Debug, Deserialize)]
pub struct SignUpPayLoad {
    pub username: String,
    pub password: String,
}

pub async fn handler(
    cookies: Cookies,
    db: ExtensionDB,
    payload: Json<SignUpPayLoad>,
) -> Result<impl IntoResponse> {
    username_exists(&db, &payload.username).await?;
    validate_payload(&payload).await?;

    let hashed_password = hash(&payload.password, DEFAULT_COST)?;
    let record = add_user(&db, &payload.username, &hashed_password).await?;

    add_auth_token(&cookies, record.id().id.to_raw());

    Ok(StatusCode::CREATED)
}
