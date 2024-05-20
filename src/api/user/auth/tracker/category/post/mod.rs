mod defs;
mod utils;

use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use surrealdb::sql::Thing;

use crate::api::{
    db::defs::{DBTables, ExtensionDB},
    user::auth::{ctx::RawUser, tracker::utils::category_ownership},
};

use self::{defs::TransactionPayload, utils::add_transaction};

pub async fn handler(
    db: ExtensionDB,
    ctx: RawUser,
    Path(id): Path<String>,
    payload: Json<TransactionPayload>,
) -> impl IntoResponse {
    let id = Thing::from((DBTables::CATEGORY, id.as_str()));

    if let Err(error) = category_ownership(&db, ctx.id(), &id).await {
        return error.into_response();
    }

    match add_transaction(&db, &id, &payload).await {
        Ok(record) => (
            StatusCode::CREATED,
            Json(json!({"id": record.id().id.to_raw()})),
        )
            .into_response(),
        Err(error) => return error.into_response(),
    }
}
