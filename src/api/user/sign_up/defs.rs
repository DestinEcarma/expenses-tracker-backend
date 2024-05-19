
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct SignUpPayLoad {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct SignUpInsert {
    pub username: String,
    pub password: String,
}
