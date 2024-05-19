use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Thing};

#[derive(Debug, Deserialize, Serialize)]
pub struct Transaction {
    id: Thing,
    amount: f64,
    description: String,
    created_at: Datetime,
}
