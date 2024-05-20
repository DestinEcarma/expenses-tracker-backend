use axum::{http::StatusCode, middleware, response::IntoResponse, routing::get, Router};

use self::utils::mw_require_auth;

mod ctx;
mod tracker;
mod utils;

pub fn router() -> Router {
    Router::new()
        .route("/", get(handler))
        .nest("/tracker", tracker::router())
        .route_layer(middleware::from_fn(mw_require_auth))
}

pub async fn handler() -> impl IntoResponse {
    (StatusCode::OK).into_response()
}
