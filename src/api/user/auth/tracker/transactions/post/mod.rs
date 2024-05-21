use crate::{
    api::{
        db::defs::{DBGlobalQuery, DBTables, ExtensionDB},
        defs::Record,
    },
    error::{Error, Result},
};
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;
use serde_json::json;
use surrealdb::sql::Thing;

#[derive(Deserialize)]
pub struct TransactionPayload {
    pub description: String,
    pub amount: f64,
}

pub async fn handler(
    db: ExtensionDB,
    Path(category_id): Path<String>,
    payload: Json<TransactionPayload>,
) -> Result<impl IntoResponse> {
    let category_id = Thing::from((DBTables::CATEGORY, category_id.as_str()));
    let record = add_transaction(&db, &category_id, &payload).await?;

    Ok((
        StatusCode::CREATED,
        Json(json!({"id": record.id().id.to_raw()})),
    ))
}

pub async fn add_transaction(
    db: &ExtensionDB,
    category_id: &Thing,
    payload: &Json<TransactionPayload>,
) -> Result<Record> {
    Ok(db
        .query(DBGlobalQuery::ADD_TRANSACTION)
        .bind(("category_id", category_id))
        .bind(("amount", &payload.amount))
        .bind(("description", &payload.description))
        .await?
        .take::<Option<Record>>(0)?
        .ok_or(Error::SurrealdbCouldNotCreateQuery)?)
}
