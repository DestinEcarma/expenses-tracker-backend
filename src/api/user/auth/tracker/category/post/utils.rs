use axum::Json;
use surrealdb::sql::Thing;

use crate::api::{
    db::defs::{DBGlobalQuery, ExtensionDB},
    defs::{Error, Record},
};

use super::defs::TransactionPayload;

pub async fn add_transaction(
    db: &ExtensionDB,
    category_id: &Thing,
    payload: &Json<TransactionPayload>,
) -> Result<Record, Error> {
    Ok(db
        .query(DBGlobalQuery::ADD_TRANSACTION)
        .bind(("category_id", category_id))
        .bind(("amount", &payload.amount))
        .bind(("description", &payload.description))
        .await
        .map_err(|e| Error::Server(e.to_string()))?
        .take::<Option<Record>>(0)
        .map_err(|e| Error::Server(e.to_string()))?
        .unwrap())
}
