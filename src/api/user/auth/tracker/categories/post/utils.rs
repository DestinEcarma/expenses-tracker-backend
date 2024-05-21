use crate::{
	api::{
		db::defs::{DBGlobalQuery, ExtensionDB},
		defs::Record,
		user::auth::tracker::categories::defs::CategoryPayload,
	},
	error::{Error, Result},
};
use axum::extract::Json;
use surrealdb::sql::Thing;

pub async fn add_category(
	db: &ExtensionDB,
	user_id: &Thing,
	payload: &Json<CategoryPayload>,
) -> Result<Record> {
	let record = db
		.query(DBGlobalQuery::ADD_CATEGORY)
		.bind(("user_id", user_id))
		.bind(("name", &payload.name))
		.bind(("icon", &payload.icon))
		.await?
		.take::<Option<Record>>(0)?
		.ok_or(Error::SurrealdbCouldNotCreateQuery)?;

	Ok(record)
}
