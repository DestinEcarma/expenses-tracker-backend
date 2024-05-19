use std::sync::Arc;

use axum::{Extension, Router};
use tower_cookies::CookieManagerLayer;

mod db;
mod defs;
mod user;

pub async fn router() -> Router {
    let db = Arc::new(db::get().await);

    Router::new()
        .nest("/user", user::router())
        .layer(CookieManagerLayer::new())
        .layer(Extension(db))
}
