mod api;
pub mod error;

use axum::{
	http::{header, HeaderValue, Method},
	Router,
};
use shuttle_runtime::Environment;
use tower_http::{
	cors::CorsLayer,
	services::{ServeDir, ServeFile},
};

#[shuttle_runtime::main]
async fn main(
	#[shuttle_runtime::Secrets] secrets: shuttle_runtime::SecretStore,
	#[shuttle_runtime::Metadata] meta: shuttle_runtime::DeploymentMetadata,
) -> shuttle_axum::ShuttleAxum {
	let router = Router::new()
		.nest("/api", api::router(secrets).await)
		.fallback_service(static_route())
		.layer(cors_middleware(meta.env));

	Ok(router.into())
}

fn static_route() -> Router {
	let serve_dir = ServeDir::new("assets").not_found_service(ServeFile::new("assets/index.html"));

	Router::new().nest_service("/", serve_dir)
}

#[rustfmt::skip]
fn cors_middleware(env: Environment) -> CorsLayer {
	let cors = CorsLayer::new()
		.allow_headers([
			header::AUTHORIZATION,
			header::CONTENT_TYPE,
			header::SET_COOKIE,
			header::COOKIE,
			header::ACCEPT,
		])
		.allow_credentials(true)
		.allow_methods([Method::POST, Method::GET, Method::DELETE, Method::PATCH]);

	match env {
		Environment::Local => cors.allow_origin("http://127.0.0.1:3000".parse::<HeaderValue>().unwrap()),
		Environment::Deployment => cors,
	}
}
