use serde_json::{json, Value};
use surrealdb::sql::Thing;

use crate::api::{
    db::defs::{DBGlobalQuery, ExtensionDB},
    defs::{merge_json, Error, RecordOut},
};

use super::defs::{Category, CategoryCal};

pub async fn get_category(db: &ExtensionDB, id: &Thing) -> Result<Value, Error> {
    let category = db
        .query(DBGlobalQuery::SELECT_CATEGORY)
        .bind(("category_id", id))
        .await
        .map_err(|e| Error::Server(e.to_string()))?
        .take::<Option<Category>>(0)
        .map_err(|e| Error::Server(e.to_string()))?
        .unwrap();

    let category = merge_json(category, get_transactions(db, id).await?);

    Ok(merge_json(category, json!({"id": id.id.to_raw()})))
}

pub async fn get_categories(db: &ExtensionDB, user_id: &Thing) -> Result<Vec<Value>, Error> {
    let categories = db
        .query(DBGlobalQuery::SELECT_USER_GET_CATEGORIES)
        .bind(("user_id", user_id))
        .await
        .map_err(|e| Error::Server(e.to_string()))?
        .take::<Option<RecordOut>>(0)
        .map_err(|e| Error::Server(e.to_string()))?
        .unwrap();

    let mut result = Vec::<Value>::new();

    for id in categories.out() {
        result.push(get_category(db, id).await?);
    }

    Ok(result)
}

async fn get_transactions(db: &ExtensionDB, category_id: &Thing) -> Result<CategoryCal, Error> {
    let transactions = db
        .query(DBGlobalQuery::SELECT_CATEGORY_GET_TRANSACTIONS)
        .bind(("category_id", category_id))
        .await
        .map_err(|e| Error::Server(e.to_string()))?
        .take::<Option<RecordOut>>(0)
        .map_err(|e| Error::Server(e.to_string()))?
        .unwrap();

    let mut amount = 0.0;

    for id in transactions.out() {
        let transaction = db
            .query(DBGlobalQuery::SELECT_TRANSACTION_AMOUNT)
            .bind(("transaction_id", id))
            .await
            .map_err(|e| Error::Server(e.to_string()))?
            .take::<Option<Value>>(0)
            .map_err(|e| Error::Server(e.to_string()))?
            .unwrap();

        amount += transaction["amount"].as_f64().unwrap();
    }

    Ok(CategoryCal::new(amount, transactions.out().len()))
}
