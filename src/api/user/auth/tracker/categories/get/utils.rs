use crate::{
    api::{
        db::defs::{DBGlobalQuery, ExtensionDB},
        defs::{merge_json, RecordOut},
        user::auth::tracker::categories::defs::Category,
    },
    error::{Error, Result},
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use surrealdb::sql::Thing;

#[derive(Debug, Deserialize, Serialize)]
pub struct CategoryCal {
    transactions: usize,
    amount: f64,
}

impl CategoryCal {
    pub fn new(amount: f64, total: usize) -> Self {
        Self {
            transactions: total,
            amount,
        }
    }
}

pub async fn get_categories(db: &ExtensionDB, user_id: &Thing) -> Result<Vec<Value>> {
    let categories = db
        .query(DBGlobalQuery::SELECT_USER_CATEGORIES)
        .bind(("user_id", user_id))
        .await?
        .take::<Option<RecordOut>>(0)?
        .ok_or(Error::SurrealdbSelectUserCategoriesIsNone)?;

    let mut result = Vec::<Value>::new();

    for id in categories.out() {
        result.push(get_category(db, id).await?);
    }

    Ok(result)
}

pub async fn get_category(db: &ExtensionDB, category_id: &Thing) -> Result<Value> {
    let category = db
        .query(DBGlobalQuery::SELECT_CATEGORY)
        .bind(("category_id", category_id))
        .await?
        .take::<Option<Category>>(0)?
        .ok_or(Error::SurrealdbSelectCategoryIsNone)?;

    Ok(merge_json(
        merge_json(category, get_transactions(db, category_id).await?),
        json!({"id": category_id.id.to_raw()}),
    ))
}

async fn get_transactions(db: &ExtensionDB, category_id: &Thing) -> Result<CategoryCal> {
    let transactions = db
        .query(DBGlobalQuery::SELECT_CATEGORY_TRANSACTIONS)
        .bind(("category_id", category_id))
        .await?
        .take::<Option<RecordOut>>(0)?
        .ok_or(Error::SurrealdbCategoryTransactionsIsNone)?;

    let mut amount = 0.0;

    for id in transactions.out() {
        let transaction = db
            .query(DBGlobalQuery::SELECT_TRANSACTION_AMOUNT)
            .bind(("transaction_id", id))
            .await?
            .take::<Option<Value>>(0)?
            .ok_or(Error::SurrealdbSelectTransactionAmountIsNone)?;

        amount += transaction["amount"].as_f64().ok_or(Error::IoError)?;
    }

    Ok(CategoryCal::new(amount, transactions.out().len()))
}
