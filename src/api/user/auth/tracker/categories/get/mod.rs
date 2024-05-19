mod utils;
mod defs;

use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::api::{db::defs::ExtensionDB, user::auth::ctx::RawUser};

use self::utils::get_categories;

pub async fn handler(db: ExtensionDB, ctx: RawUser) -> impl IntoResponse {
    let categories = match get_categories(&db, ctx.id()).await {
        Err(error) => return error.into_response(),
        Ok(categories) => categories,
    };

    (StatusCode::OK, Json(categories)).into_response()
}
