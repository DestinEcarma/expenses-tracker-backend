mod utils;

use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use surrealdb::sql::Thing;

use crate::api::{
    db::defs::{DBTables, ExtensionDB},
    user::auth::{
        ctx::RawUser,
        tracker::{
            categories::{patch::utils::update_category, utils::validate_payload},
            utils::category_ownership,
        },
    },
};

use super::defs::CategoryPayload;

pub async fn handler(
    db: ExtensionDB,
    ctx: RawUser,
    Path(id): Path<String>,
    payload: Json<CategoryPayload>,
) -> impl IntoResponse {
    let id = Thing::from((DBTables::CATEGORY, id.as_str()));

    if let Err(error) = category_ownership(&db, ctx.id(), &id).await {
        return error.into_response();
    }

    if let Err(error) = validate_payload(&payload).await {
        return error.into_response();
    }

    if let Err(error) = update_category(&db, &id, &payload).await {
        return error.into_response();
    }

    (StatusCode::OK).into_response()
}
