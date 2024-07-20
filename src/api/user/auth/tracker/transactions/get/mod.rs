use crate::{
	api::db::defs::{DBGlobalQuery, DBTables, ExtensionDB},
	error::Result,
};
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;
use surrealdb::sql::{Datetime, Thing};

#[derive(Debug, Deserialize, Serialize)]
struct Transaction {
	id: String,
	amount: f64,
	description: String,
	created_at: Datetime,
}

#[derive(Debug, Deserialize, Serialize)]
struct Category {
	name: String,
	icon: String,
}

pub async fn handler(
	db: ExtensionDB,
	Path(category_id): Path<String>,
) -> Result<impl IntoResponse> {
	let category_id = Thing::from((DBTables::CATEGORY, category_id.as_str()));

	let mut result = db
		.query(DBGlobalQuery::SELECT_CATEGORY_TRANSACTIONS)
		.query(DBGlobalQuery::SELECT_CATEGORY)
		.bind(("category_id", category_id))
		.await?;

	let transactions = result.take::<Vec<Transaction>>(0)?;
	let category = result.take::<Option<Category>>(1)?.unwrap();

	Ok((
		StatusCode::OK,
		Json(json!({
			"category": category,
			"transactions": transactions
		})),
	))
}
