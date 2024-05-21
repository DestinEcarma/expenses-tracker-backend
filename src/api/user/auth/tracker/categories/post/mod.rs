mod utils;

use super::{defs::CategoryPayload, utils::validate_payload};
use crate::{
    api::{
        db::defs::ExtensionDB,
        user::auth::{ctx::UserId, tracker::categories::post::utils::add_category},
    },
    error::Result,
};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

pub async fn handler(
    db: ExtensionDB,
    ctx: UserId,
    payload: Json<CategoryPayload>,
) -> Result<impl IntoResponse> {
    validate_payload(&payload).await?;

    let record = add_category(&db, ctx.id(), &payload).await?;

    Ok((
        StatusCode::CREATED,
        Json(json!({"id": record.id().id.to_raw()})),
    ))
}
