mod defs;
mod utils;

use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use surrealdb::sql::Thing;

use crate::api::{
    db::defs::{DBTables, ExtensionDB},
    user::auth::{ctx::RawUser, tracker::utils::category_ownership},
};

use self::utils::{get_categories, get_category};

pub async fn handler_many(db: ExtensionDB, ctx: RawUser) -> impl IntoResponse {
    let categories = match get_categories(&db, ctx.id()).await {
        Err(error) => return error.into_response(),
        Ok(categories) => categories,
    };

    (StatusCode::OK, Json(categories)).into_response()
}

pub async fn handler_one(
    db: ExtensionDB,
    ctx: RawUser,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let id = Thing::from((DBTables::CATEGORY, id.as_str()));

    if let Err(error) = category_ownership(&db, ctx.id(), &id).await {
        return error.into_response();
    }

    let category = match get_category(&db, &id).await {
        Err(error) => return error.into_response(),
        Ok(category) => category,
    };

    (StatusCode::OK, Json(category)).into_response()
}
