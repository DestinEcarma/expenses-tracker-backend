use crate::{
	api::db::defs::{DBGlobalQuery, ExtensionDB},
	error::{Error, Result},
};
use bcrypt::verify;
use serde::Deserialize;
use surrealdb::sql::Thing;

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

pub async fn get_user(db: &ExtensionDB, username: &String) -> Result<UserLogin> {
	db.query(DBGlobalQuery::SELECT_USER_BY_USERNAME)
		.bind(("username", username))
		.await?
		.take::<Option<UserLogin>>(0)?
		.ok_or(Error::InvalidUserPayload)
}

pub async fn verify_password(password: &str, hashed_password: &str) -> Result<()> {
	match verify(password, hashed_password)? {
		false => Err(Error::InvalidUserPayload),
		true => Ok(()),
	}
}
