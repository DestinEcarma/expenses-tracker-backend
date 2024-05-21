use crate::{
    api::db::defs::{DBGlobalQuery, DBTables, ExtensionDB},
    error::Result,
};
use axum::{extract::Path, http::StatusCode, response::IntoResponse};
use surrealdb::sql::Thing;

pub async fn handler(db: ExtensionDB, Path(id): Path<String>) -> Result<impl IntoResponse> {
    let id = Thing::from((DBTables::CATEGORY, id.as_str()));

    delete_category(&db, &id).await?;

    Ok(StatusCode::OK)
}

pub async fn delete_category(db: &ExtensionDB, category_id: &Thing) -> Result<()> {
    db.query(DBGlobalQuery::DELETE_CATEGORY)
        .bind(("category_id", category_id))
        .await?;

    Ok(())
}
