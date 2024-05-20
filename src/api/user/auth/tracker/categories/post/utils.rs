use axum::extract::Json;

use surrealdb::sql::Thing;

use crate::api::{
    db::defs::{DBGlobalQuery, ExtensionDB},
    defs::{Error, Record},
    user::auth::tracker::categories::defs::CategoryPayload,
};

pub async fn add_category(
    db: &ExtensionDB,
    user_id: &Thing,
    payload: &Json<CategoryPayload>,
) -> Result<Record, Error> {
    Ok(db
        .query(DBGlobalQuery::ADD_CATEGORY)
        .bind(("user_id", user_id))
        .bind(("name", &payload.name))
        .bind(("icon", &payload.icon))
        .await
        .map_err(|e| Error::Server(e.to_string()))?
        .take::<Option<Record>>(0)
        .map_err(|e| Error::Server(e.to_string()))?
        .unwrap())
}
