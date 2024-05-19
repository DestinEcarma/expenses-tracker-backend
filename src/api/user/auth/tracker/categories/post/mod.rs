mod defs;
mod utils;

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

use crate::api::{
    db::defs::ExtensionDB,
    user::auth::{
        ctx::RawUser,
        tracker::categories::post::utils::{add_category, validate_payload},
    },
};

use self::defs::CategoryPayload;

pub async fn handler(
    db: ExtensionDB,
    ctx: RawUser,
    payload: Json<CategoryPayload>,
) -> impl IntoResponse {
    if let Err(error) = validate_payload(&payload).await {
        return error.into_response();
    }

    match add_category(&db, ctx.id(), &payload).await {
        Err(error) => return error.into_response(),
        Ok(record) => (
            StatusCode::CREATED,
            Json(json!({"id": record.id().id.to_raw()})),
        )
            .into_response(),
    }
}
