pub mod defs;
mod delete;
mod get;
mod patch;
mod post;
mod utils;

use axum::{
	routing::{self},
	Router,
};

#[rustfmt::skip]
pub fn router() -> Router {
	Router::new()
		.route("/", routing::get(get::handler).post(post::handler))
		.route("/:id", routing::patch(patch::handler).delete(delete::handler))
}
