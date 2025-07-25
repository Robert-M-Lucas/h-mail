pub mod interface;
mod routes;

use std::net::SocketAddr;
use crate::root::receiving::routes::deliver_email::deliver_email;
use crate::root::receiving::routes::get_emails::get_emails;
use axum::routing::post;
use axum::{Router, routing::get, ServiceExt};
use routes::check_pow::check_pow;
use routes::pow_request::pow_request;

pub async fn comm_main_blocking() {
    println!("Starting listener");

    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/pow_request", get(pow_request))
        .route("/check_pow", get(check_pow))
        .route("/deliver_email", post(deliver_email))
        .route("/get_emails", get(get_emails));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8081").await.unwrap();
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}
