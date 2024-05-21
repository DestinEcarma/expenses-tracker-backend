mod utils;

use self::utils::delete_transaction;
use super::utils::transaction_ownership;
use crate::{
    api::db::defs::{DBTables, ExtensionDB},
    error::Result,
};
use axum::{extract::Path, http::StatusCode, response::IntoResponse};
use surrealdb::sql::Thing;

pub async fn handler(
    db: ExtensionDB,
    Path((category_id, transaction_id)): Path<(String, String)>,
) -> Result<impl IntoResponse> {
    let category_id = Thing::from((DBTables::CATEGORY, category_id.as_str()));
    let transaction_id = Thing::from((DBTables::TRANSACTION, transaction_id.as_str()));

    transaction_ownership(&db, &category_id, &transaction_id).await?;
    delete_transaction(&db, &transaction_id).await?;

    Ok(StatusCode::OK)
}
