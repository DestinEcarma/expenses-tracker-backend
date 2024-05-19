mod defs;
mod utils;

use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use surrealdb::sql::Thing;

use crate::api::{
    db::defs::ExtensionDB,
    user::auth::{ctx::RawUser, tracker::utils::category_ownership},
};

use self::utils::get_transactions;

pub async fn handler(db: ExtensionDB, ctx: RawUser, Path(id): Path<String>) -> impl IntoResponse {
    let id = Thing::from(("category", id.as_str()));

    if let Err(error) = category_ownership(&db, ctx.id(), &id).await {
        return error.into_response();
    }

    match get_transactions(&db, &id).await {
        Ok(transactions) => (StatusCode::OK, Json(transactions)).into_response(),
        Err(error) => error.into_response(),
    }
}
