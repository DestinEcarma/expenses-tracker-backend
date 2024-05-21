mod categories;
mod transactions;
mod utils;

use axum::Router;

pub fn router() -> Router {
    Router::new()
        .nest("/categories", categories::router())
        .nest("/category/:id/transactions", transactions::router())
}
