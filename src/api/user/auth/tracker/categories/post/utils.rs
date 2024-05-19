use axum::extract::Json;
use regex::Regex;

use surrealdb::sql::Thing;

use crate::api::{
    db::defs::{DBGlobalQuery, ExtensionDB},
    defs::{Error, Record},
};

use super::defs::CategoryPayload;

static CATEGORY_NAME_REGEX: &str = r"^.{3,}$";

pub async fn validate_payload(payload: &Json<CategoryPayload>) -> Result<(), Error> {
    let name_regex = Regex::new(CATEGORY_NAME_REGEX).map_err(|e| Error::Server(e.to_string()))?;

    match name_regex.is_match(&payload.name) {
        false => Err(Error::InvalidCategoryPayload),
        true => Ok(()),
    }
}

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
