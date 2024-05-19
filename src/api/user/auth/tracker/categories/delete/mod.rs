mod utils;

use axum::{extract::Path, http::StatusCode, response::IntoResponse};
use surrealdb::sql::Thing;

use crate::api::{
    db::defs::ExtensionDB,
    user::auth::{ctx::RawUser, tracker::utils::category_ownership},
};

use self::utils::delete_category;

pub async fn handler(db: ExtensionDB, ctx: RawUser, Path(id): Path<String>) -> impl IntoResponse {
    let id = Thing::from(("category", id.as_str()));

    if let Err(error) = category_ownership(&db, ctx.id(), &id).await {
        return error.into_response();
    }

    if let Err(error) = delete_category(&db, &id).await {
        return error.into_response();
    }

    (StatusCode::OK).into_response()
}
