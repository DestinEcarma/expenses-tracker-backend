mod categories;
mod category;
mod utils;

use axum::Router;

pub fn router() -> Router {
    Router::new()
        .nest("/categories", categories::router())
        .nest("/category/:id", category::router())
}
