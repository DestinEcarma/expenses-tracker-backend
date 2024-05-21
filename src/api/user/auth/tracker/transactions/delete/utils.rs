use crate::{
	api::db::defs::{DBGlobalQuery, ExtensionDB},
	error::Result,
};
use surrealdb::sql::Thing;

pub async fn delete_transaction(db: &ExtensionDB, transaction_id: &Thing) -> Result<()> {
	db.query(DBGlobalQuery::DELETE_TRANSACTION)
		.bind(("transaction_id", transaction_id))
		.await?;

	Ok(())
}
