use serde_json::{json, Value};
use surrealdb::sql::Thing;

use crate::api::{
    db::defs::{DBGlobalQuery, ExtensionDB},
    defs::{merge_json, Error, RecordOut},
};

use super::defs::Transaction;

pub async fn get_transactions(db: &ExtensionDB, category: &Thing) -> Result<Vec<Value>, Error> {
    let transactions = db
        .query(DBGlobalQuery::SELECT_CATEGORY_GET_TRANSACTIONS)
        .bind(("category_id", category))
        .await
        .map_err(|e| Error::Server(e.to_string()))?
        .take::<Option<RecordOut>>(0)
        .map_err(|e| Error::Server(e.to_string()))?
        .unwrap();

    let mut result = Vec::<Value>::new();

    for transaction in transactions.out() {
        result.push(merge_json(
            json!(db
                .query(DBGlobalQuery::SELECT_TRANSACTION_GET)
                .bind(("transaction_id", transaction))
                .await
                .map_err(|e| Error::Server(e.to_string()))?
                .take::<Option<Transaction>>(0)
                .map_err(|e| Error::Server(e.to_string()))?
                .unwrap()),
            json!({ "id": transaction.id.to_raw() }),
        ));
    }

    Ok(result)
}
