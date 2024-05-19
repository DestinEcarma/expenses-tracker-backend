use serde::Deserialize;

#[derive(Deserialize)]
pub struct TransactionPayload {
    pub description: String,
    pub amount: f64,
}
