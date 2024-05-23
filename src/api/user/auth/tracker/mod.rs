mod categories;
mod transactions;
mod utils;

use axum::Router;

#[rustfmt::skip]
pub fn router() -> Router {
	Router::new()
		.nest("/categories", categories::router())
		.nest("/category/:category_id/transactions", transactions::router())
}
