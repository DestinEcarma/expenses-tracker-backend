mod delete;
mod get;
mod post;

use axum::{
    routing::{self},
    Router,
};

pub fn router() -> Router {
    Router::new()
        .route("/", routing::get(get::handler).post(post::handler))
        .route("/:id", routing::delete(delete::handler))
}
