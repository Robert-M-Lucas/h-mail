pub mod interface;
mod routes;

use axum::{Router, routing::get};
use axum::routing::post;
use routes::check_pow::check_pow;
use routes::pow_request::pow_request;
use crate::root::communication::routes::send_email::send_email;

pub async fn comm_main_blocking() {
    println!("Starting listener");

    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/pow_request", get(pow_request))
        .route("/check_pow", get(check_pow))
        .route("/send_email", post(send_email));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8081").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}
