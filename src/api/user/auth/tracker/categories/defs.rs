use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CategoryPayload {
    pub name: String,
    pub icon: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Category {
    name: String,
    icon: String,
}
