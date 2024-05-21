use crate::{
	api::db::defs::{DBGlobalQuery, ExtensionDB},
	error::{Error, Result},
};
use surrealdb::sql::Thing;

pub async fn category_ownership(
	db: &ExtensionDB,
	user_id: &Thing,
	category_id: &Thing,
) -> Result<()> {
	let is_owned = db
		.query(DBGlobalQuery::CATEGORY_OWNERSHIP)
		.bind(("category_id", category_id))
		.bind(("user_id", user_id))
		.await?
		.take::<Option<bool>>(0)?
		.ok_or(Error::SurrealdbCategoryOwnershipIsNone)?;

	match is_owned {
		false => Err(Error::NotFound),
		true => Ok(()),
	}
}
