mod ctx;
mod tracker;
mod utils;

use self::utils::mw_require_auth;
use crate::error::Result;
use axum::{http::StatusCode, middleware, response::IntoResponse, routing::get, Router};

pub fn router() -> Router {
    Router::new()
        .route("/", get(handler))
        .nest("/tracker", tracker::router())
        .route_layer(middleware::from_fn(mw_require_auth))
}

pub async fn handler() -> Result<impl IntoResponse> {
    Ok(StatusCode::OK)
}
