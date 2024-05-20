mod defs;
mod delete;
mod get;
mod patch;
mod post;
mod utils;

use axum::{
    routing::{self},
    Router,
};

pub fn router() -> Router {
    Router::new()
        .route("/", routing::get(get::handler_many).post(post::handler))
        .route(
            "/:id",
            routing::get(get::handler_one)
                .patch(patch::handler)
                .delete(delete::handler),
        )
}
