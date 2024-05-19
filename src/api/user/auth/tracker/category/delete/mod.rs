mod utils;

use axum::{extract::Path, http::StatusCode, response::IntoResponse};
use surrealdb::sql::Thing;

use crate::api::{
    db::defs::ExtensionDB,
    user::auth::{
        ctx::RawUser,
        tracker::utils::{category_ownership, transaction_ownership},
    },
};

use self::utils::delete_transaction;

pub async fn handler(
    db: ExtensionDB,
    ctx: RawUser,
    Path((category_id, transaction_id)): Path<(String, String)>,
) -> impl IntoResponse {
    let category_id = Thing::from(("category", category_id.as_str()));
    let transaction_id = Thing::from(("transaction", transaction_id.as_str()));

    if let Err(error) = category_ownership(&db, ctx.id(), &category_id).await {
        return error.into_response();
    }

    if let Err(error) = transaction_ownership(&db, &category_id, &transaction_id).await {
        return error.into_response();
    }

    if let Err(error) = delete_transaction(&db, &transaction_id).await {
        return error.into_response();
    }

    (StatusCode::OK).into_response()
}
