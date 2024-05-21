mod utils;

use self::utils::get_categories;
use crate::{
    api::{db::defs::ExtensionDB, user::auth::ctx::UserId},
    error::Result,
};
use axum::{http::StatusCode, response::IntoResponse, Json};

pub async fn handler(db: ExtensionDB, ctx: UserId) -> Result<impl IntoResponse> {
    Ok((StatusCode::OK, Json(get_categories(&db, ctx.id()).await?)))
}
