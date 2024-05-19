use surrealdb::sql::Thing;

use crate::api::{
    db::defs::{DBGlobalQuery, ExtensionDB},
    defs::Error,
};

pub async fn delete_category(db: &ExtensionDB, category_id: &Thing) -> Result<(), Error> {
    db.query(DBGlobalQuery::DELETE_CATEGORY)
        .bind(("category_id", category_id))
        .await
        .map_err(|e| Error::Server(e.to_string()))?;

    Ok(())
}
