use crate::{
	api::{
		db::defs::{DBGlobalQuery, ExtensionDB},
		user::auth::ctx::UserId,
	},
	error::Result,
};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Deserialize, Serialize)]
struct Category {
	id: Thing,
	name: String,
	icon: String,
	amount: f64,
	transactions: usize,
}

pub async fn handler(db: ExtensionDB, ctx: UserId) -> Result<impl IntoResponse> {
	let categories = db
		.query(DBGlobalQuery::SELECT_USER_CATEGORIES)
		.bind(("user_id", ctx.id()))
		.await?
		.take::<Vec<Category>>(0)?;

	Ok((StatusCode::OK, Json(categories)))
}
