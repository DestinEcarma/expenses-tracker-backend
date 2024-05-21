mod delete;
mod get;
mod post;
mod utils;

use super::utils::category_ownership;
use crate::{
	api::{
		db::defs::{DBTables, ExtensionDB},
		user::auth::ctx::UserId,
	},
	error::Result,
};
use axum::{
	extract::{Path, Request},
	middleware::{self, Next},
	response::Response,
	routing, Router,
};
use surrealdb::sql::Thing;

#[rustfmt::skip]
pub fn router() -> Router {
	Router::new()
		.route("/", routing::get(get::handler).post(post::handler))
		.route("/:id", routing::delete(delete::handler))
		.route_layer(middleware::from_fn(mw_category_ownership))
}

async fn mw_category_ownership(
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
