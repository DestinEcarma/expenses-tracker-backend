use crate::{
	api::{
		db::defs::{DBGlobalQuery, DBTables, ExtensionDB},
		defs::{merge_json, RecordOut},
		user::auth::tracker::categories::defs::Category,
	},
	error::{Error, Result},
};
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use surrealdb::sql::{Datetime, Thing};

#[derive(Debug, Deserialize, Serialize)]
struct Transaction {
	id: Thing,
	amount: f64,
	description: String,
	created_at: Datetime,
}

pub async fn handler(
	db: ExtensionDB,
	Path(category_id): Path<String>,
) -> Result<impl IntoResponse> {
	let category_id = Thing::from((DBTables::CATEGORY, category_id.as_str()));

	Ok((
		StatusCode::OK,
		Json(get_category_transactions(&db, &category_id).await?),
	))
}

pub async fn get_category_transactions(db: &ExtensionDB, category: &Thing) -> Result<Value> {
	let mut result = db
		.query(DBGlobalQuery::SELECT_CATEGORY_TRANSACTIONS)
		.query(DBGlobalQuery::SELECT_CATEGORY)
		.bind(("category_id", category))
		.await?;

	let transactions = result.take::<Option<RecordOut>>(0)?.unwrap();
	let category = result.take::<Option<Category>>(1)?.unwrap();

	let mut result = Vec::<Value>::new();

	for transaction in transactions.out() {
		result.push(merge_json(
			json!(db
				.query(DBGlobalQuery::SELECT_TRANSACTION)
				.bind(("transaction_id", transaction))
				.await?
				.take::<Option<Transaction>>(0)?
				.ok_or(Error::SurrealdbSelectTransactionIsNone)?),
			json!({ "id": transaction.id.to_raw() }),
		));
	}

	Ok(json!({
		"category": category,
		"transactions": result
	}))
}
