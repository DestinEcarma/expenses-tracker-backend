mod utils;

use self::utils::delete_transaction;
use crate::{
	api::db::defs::{DBTables, ExtensionDB},
	error::Result,
};
use axum::{extract::Path, http::StatusCode, response::IntoResponse};
use surrealdb::sql::Thing;

pub async fn handler(
	db: ExtensionDB,
	Path((_, transaction_id)): Path<(String, String)>,
) -> Result<impl IntoResponse> {
	let transaction_id = Thing::from((DBTables::TRANSACTION, transaction_id.as_str()));

	delete_transaction(&db, &transaction_id).await?;

	Ok(StatusCode::OK)
}
