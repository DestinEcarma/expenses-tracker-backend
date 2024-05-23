mod delete;
mod get;
mod post;
mod utils;

use self::utils::{mw_category_ownership, mw_transaction_ownership};
use axum::{middleware, routing, Router};

pub fn router() -> Router {
	Router::new()
		.merge(
			Router::new()
				.route("/", routing::get(get::handler).post(post::handler))
				.route_layer(middleware::from_fn(mw_category_ownership)),
		)
		.merge(
			Router::new()
				.route("/:transaction_id", routing::delete(delete::handler))
				.route_layer(middleware::from_fn(mw_transaction_ownership)),
		)
}
