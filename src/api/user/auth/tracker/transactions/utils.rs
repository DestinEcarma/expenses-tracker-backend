use crate::{
    api::db::defs::{DBGlobalQuery, ExtensionDB},
    error::{Error, Result},
};
use surrealdb::sql::Thing;

pub async fn transaction_ownership(
    db: &ExtensionDB,
    category_id: &Thing,
    transaction_id: &Thing,
) -> Result<()> {
    let is_owned = db
        .query(DBGlobalQuery::TRANSACTION_OWNERSHIP)
        .bind(("transaction_id", transaction_id))
        .bind(("category_id", category_id))
        .await?
        .take::<Option<bool>>(0)?
        .ok_or(Error::SurrealdbTransactionOwnershipIsNone)?;

    match is_owned {
        false => Err(Error::NotFound),
        true => Ok(()),
    }
}
