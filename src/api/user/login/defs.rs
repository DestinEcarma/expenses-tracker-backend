use serde::Deserialize;
use surrealdb::sql::Thing;

#[derive(Debug, Deserialize)]
pub struct LoginPayLoad {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UserLogin {
    id: Thing,
    password: String,
}

impl UserLogin {
    pub fn password(&self) -> &String {
        &self.password
    }

    pub fn id(&self) -> &Thing {
        &self.id
    }
}
