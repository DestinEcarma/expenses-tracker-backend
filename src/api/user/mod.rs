mod auth;
mod login;
mod logout;
mod sign_up;
mod utils;

use axum::{
    routing::{delete, post},
    Router,
};

pub fn router() -> Router {
    Router::new()
        .route("/login", post(login::handler))
        .route("/sign-up", post(sign_up::handler))
        .route("/logout", delete(logout::handler))
        .nest("/auth", auth::router())
}
