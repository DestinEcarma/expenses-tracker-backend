mod defs;
mod utils;

use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use tower_cookies::Cookies;

use self::{
    defs::SignUpPayLoad,
    utils::{add_user, hash_password, username_exists, validate_payload},
};
use crate::api::db::defs::ExtensionDB;

use super::utils::add_auth_token;

pub async fn handler(
    cookies: Cookies,
    db: ExtensionDB,
    payload: Json<SignUpPayLoad>,
) -> impl IntoResponse {
    if let Err(error) = username_exists(&db, &payload.username).await {
        return error.into_response();
    }

    if let Err(error) = validate_payload(&payload).await {
        return error.into_response();
    }

    let hashed_password = match hash_password(&payload.password).await {
        Err(error) => return error.into_response(),
        Ok(hashed_password) => hashed_password,
    };

    let record = match add_user(&db, &payload.username, &hashed_password).await {
        Err(error) => return error.into_response(),
        Ok(record) => record,
    };

    add_auth_token(&cookies, record.id().id.to_raw());

    (StatusCode::OK).into_response()
}
