use crate::{
	api::{
		db::defs::{DBGlobalQuery, DBTables, ExtensionDB},
		user::auth::{ctx::UserId, tracker::utils::category_ownership},
	},
	error::{Error, Result},
};
use axum::{
	extract::{Path, Request},
	middleware::Next,
	response::Response,
};
use surrealdb::sql::Thing;

pub async fn mw_category_ownership(
	db: ExtensionDB,
	ctx: UserId,
	Path(category_id): Path<String>,
	req: Request,
	next: Next,
) -> Result<Response> {
	let category_id = Thing::from((DBTables::CATEGORY, category_id.as_str()));

	category_ownership(&db, ctx.id(), &category_id).await?;

	Ok(next.run(req).await)
}

pub async fn mw_transaction_ownership(
	db: ExtensionDB,
	ctx: UserId,
	Path((category_id, transaction_id)): Path<(String, String)>,
	req: Request,
	next: Next,
) -> Result<Response> {
	let category_id = Thing::from((DBTables::CATEGORY, category_id.as_str()));
	let transaction_id = Thing::from((DBTables::TRANSACTION, transaction_id.as_str()));

	category_ownership(&db, ctx.id(), &category_id).await?;
	transaction_ownership(&db, &category_id, &transaction_id).await?;

	Ok(next.run(req).await)
}

pub async fn transaction_ownership(
	db: &ExtensionDB,
	category_id: &Thing,
	transaction_id: &Thing,
) -> Result<()> {
	let is_owned = db
		.query(DBGlobalQuery::TRANSACTION_OWNERSHIP)
		.bind(("transaction_id", transaction_id))
		.bind(("category_id", category_id))
		.await?
		.take::<Option<bool>>(0)?
		.ok_or(Error::SurrealdbTransactionOwnershipIsNone)?;

	match is_owned {
		false => Err(Error::NotFound),
		true => Ok(()),
	}
}
