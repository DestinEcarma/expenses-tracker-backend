use super::ctx::UserId;
use crate::{
	api::{
		db::defs::{DBGlobalQuery, DBTables, ExtensionDB},
		defs::{CookieNames, Record},
	},
	error::{Error, Result},
};
use axum::{
	async_trait,
	extract::{FromRequestParts, Request},
	http::request::Parts,
	middleware::Next,
	response::Response,
	RequestPartsExt,
};
use surrealdb::sql::Thing;
use tower_cookies::Cookies;

pub async fn mw_require_auth(ctx: Result<UserId>, req: Request, next: Next) -> Result<Response> {
	ctx?;

	Ok(next.run(req).await)
}

#[async_trait]
impl<T: Send + Sync> FromRequestParts<T> for UserId {
	type Rejection = Error;

	async fn from_request_parts(parts: &mut Parts, _state: &T) -> Result<Self> {
		let cookies = parts.extract::<Cookies>().await.unwrap();
		let db = parts.extract::<ExtensionDB>().await.unwrap();

		let auth_token = cookies
			.get(CookieNames::AUTH_TOKEN)
			.map(|c| c.value().to_string());

		let id = auth_token.ok_or(Error::AuthFailNoAuthTokenCookie)?;
		let user_id = Thing::from((DBTables::USER, id.as_str()));

		get_user(&db, &user_id).await?;

		Ok(UserId::new(user_id))
	}
}

pub async fn get_user(db: &ExtensionDB, user_id: &Thing) -> Result<()> {
	match db
		.query(DBGlobalQuery::SELECT_USER)
		.bind(("user_id", user_id))
		.await?
		.take::<Option<Record>>(0)?
	{
		None => Err(Error::AuthFailNoAuthTokenCookie),
		Some(_) => Ok(()),
	}
}
