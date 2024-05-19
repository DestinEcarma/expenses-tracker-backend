mod api;

use axum::Router;
use dotenv::dotenv;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    dotenv().ok();

    let router = Router::new().nest("/api", api::router().await);

    Ok(router.into())
}

// #[tokio::main]
// async fn main() {
//     dotenv().ok();

//     let router = Router::new().nest("/api", api::router().await);

//     let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
//         .await
//         .unwrap();

//     axum::serve(listener, router).await.unwrap();
// }
