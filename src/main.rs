mod api;

use axum::{
    http::{header, Method},
    Router,
};
use tower_http::{
    cors::CorsLayer,
    services::{ServeDir, ServeFile},
};

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secrets: shuttle_runtime::SecretStore,
) -> shuttle_axum::ShuttleAxum {
    let cors_middleware = CorsLayer::new()
        // .allow_origin("http://127.0.0.1:3000".parse::<HeaderValue>().unwrap())
        .allow_headers([
            header::AUTHORIZATION,
            header::CONTENT_TYPE,
            header::SET_COOKIE,
            header::COOKIE,
            header::ACCEPT,
        ])
        .allow_credentials(true)
        .allow_methods([Method::POST, Method::GET, Method::DELETE, Method::PATCH]);

    let router = Router::new()
        .nest("/api", api::router(secrets).await)
        .fallback_service(static_route())
        .layer(cors_middleware);

    Ok(router.into())
}

fn static_route() -> Router {
    Router::new().nest_service(
        "/",
        ServeDir::new("assets").not_found_service(ServeFile::new("assets/index.html")),
    )
}
