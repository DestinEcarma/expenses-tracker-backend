use surrealdb::sql::Thing;

use crate::api::{
    db::defs::{DBGlobalQuery, ExtensionDB},
    defs::Error,
};

pub async fn category_ownership(
    db: &ExtensionDB,
    user_id: &Thing,
    category_id: &Thing,
) -> Result<(), Error> {
    let is_owned = db
        .query(DBGlobalQuery::CATEGORY_OWNERSHIP)
        .bind(("user_id", user_id))
        .bind(("category_id", category_id))
        .await
        .map_err(|e| Error::Server(e.to_string()))?
        .take::<Option<bool>>(0)
        .map_err(|e| Error::Server(e.to_string()))?
        .unwrap();

    match is_owned {
        false => Err(Error::NotFound),
        true => Ok(()),
    }
}

pub async fn transaction_ownership(
    db: &ExtensionDB,
    category_id: &Thing,
    transaction_id: &Thing,
) -> Result<(), Error> {
    let is_owned = db
        .query(DBGlobalQuery::TRANSACTION_OWNERSHIP)
        .bind(("category_id", category_id))
        .bind(("transaction_id", transaction_id))
        .await
        .map_err(|e| Error::Server(e.to_string()))?
        .take::<Option<bool>>(0)
        .map_err(|e| Error::Server(e.to_string()))?
        .unwrap();

    match is_owned {
        false => Err(Error::NotFound),
        true => Ok(()),
    }
}
