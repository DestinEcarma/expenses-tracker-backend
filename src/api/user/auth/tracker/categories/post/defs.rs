use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CategoryPayload {
    pub name: String,
    pub icon: String,
}
