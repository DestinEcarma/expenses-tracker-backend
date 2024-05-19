mod delete;
mod get;
mod post;

use axum::{routing, Router};

pub fn router() -> Router {
    Router::new()
        .route(
            "/transactions",
            routing::get(get::handler).post(post::handler),
        )
        .route("/transactions/:id", routing::delete(delete::handler))
}
