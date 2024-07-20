use serde::Deserialize;
use surrealdb::sql::Thing;

#[derive(Debug, Deserialize)]
pub struct Record {
	id: Thing,
}

impl Record {
	pub fn id(&self) -> &Thing {
		&self.id
	}
}

pub struct CookieNames;

impl CookieNames {
	pub const AUTH_TOKEN: &'static str = "auth-token";
}
