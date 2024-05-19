use axum::{middleware, Router};

use self::utils::mw_require_auth;

mod ctx;
mod tracker;
mod utils;

pub fn router() -> Router {
    Router::new()
        .nest("/tracker", tracker::router())
        .route_layer(middleware::from_fn(mw_require_auth))
}
