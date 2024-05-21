use super::defs::CategoryPayload;
use crate::{
	api::{
		db::defs::{DBGlobalQuery, DBTables, ExtensionDB},
		user::auth::{
			ctx::UserId,
			tracker::{categories::utils::validate_payload, utils::category_ownership},
		},
	},
	error::Result,
};
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use surrealdb::sql::Thing;

pub async fn handler(
	db: ExtensionDB,
	ctx: UserId,
	Path(id): Path<String>,
	payload: Json<CategoryPayload>,
) -> Result<impl IntoResponse> {
	let id = Thing::from((DBTables::CATEGORY, id.as_str()));

	category_ownership(&db, ctx.id(), &id).await?;
	validate_payload(&payload).await?;
	update_category(&db, &id, &payload).await?;

	Ok(StatusCode::OK)
}

pub async fn update_category(
	db: &ExtensionDB,
	category_id: &Thing,
	payload: &CategoryPayload,
) -> Result<()> {
	db.query(DBGlobalQuery::UPDATE_CATEGORY)
		.bind(("category_id", category_id))
		.bind(("name", &payload.name))
		.bind(("icon", &payload.icon))
		.await?;

	Ok(())
}
