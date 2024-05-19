use surrealdb::sql::Thing;

use crate::api::{
    db::defs::{DBGlobalQuery, ExtensionDB},
    defs::Error,
};

pub async fn delete_transaction(db: &ExtensionDB, transaction_id: &Thing) -> Result<(), Error> {
    db.query(DBGlobalQuery::DELETE_TRANSACTION)
        .bind(("transaction_id", transaction_id))
        .await
        .map_err(|e| Error::Server(e.to_string()))?;

    Ok(())
}
