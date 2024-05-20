use surrealdb::sql::Thing;

use crate::api::{
    db::defs::{DBGlobalQuery, ExtensionDB},
    defs::Error,
    user::auth::tracker::categories::defs::CategoryPayload,
};

pub async fn update_category(
    db: &ExtensionDB,
    category_id: &Thing,
    payload: &CategoryPayload,
) -> Result<(), Error> {
    db.query(DBGlobalQuery::UPDATE_CATEGORY)
        .bind(("category_id", category_id))
        .bind(("name", &payload.name))
        .bind(("icon", &payload.icon))
        .await
        .map_err(|e| Error::Server(e.to_string()))?;

    Ok(())
}
