pub mod interface;
mod routes;

use routes::pow_request::pow_request;
use axum::{
    routing::get
    , Router,
};
use routes::check_pow::check_pow;

pub async fn comm_main_blocking() {
    println!("Starting listener");

    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/pow_request", get(pow_request))
        .route("/check_pow", get(check_pow));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}