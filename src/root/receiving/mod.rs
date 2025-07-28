mod auth_util;
pub mod interface;
mod routes;

use crate::root::receiving::routes::auth::authenticate::authenticate;
use crate::root::receiving::routes::auth::refresh_access::refresh_access;
use crate::root::receiving::routes::foreign::verify_ip::verify_ip;
use auth_util::auth_header::AuthorizationHeader;
use axum::extract::ConnectInfo;
use axum::routing::post;
use axum::{Router, extract::Request, routing::get};
use hyper::body::Incoming;
use hyper::service::HttpService;
use hyper_util::rt::{TokioExecutor, TokioIo};
use routes::check_pow::check_pow;
use routes::foreign::deliver_email::deliver_email;
use routes::foreign::get_pow_token::pow_request;
use routes::foreign::get_user_pow_policy::get_user_pow_policy;
use routes::native::create_account::create_account;
use routes::native::get_create_account_pow_policy::get_create_account_pow_policy;
use routes::native::get_emails::get_emails;
use std::net::SocketAddr;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::net::TcpListener;
use tokio_rustls::{
    rustls::ServerConfig,
    rustls::pki_types::{CertificateDer, PrivateKeyDer, pem::PemObject},
};
use tower_service::Service;
use tracing::{error, warn};

pub async fn recv_main_blocking() {
    println!("Starting listener");

    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");

    let rustls_config = rustls_server_config(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("self_signed_certs")
            .join("key.pem"),
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("self_signed_certs")
            .join("cert.pem"),
    );

    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/check_pow", post(check_pow))
        .route("/foreign/pow_request", get(pow_request))
        .route("/foreign/get_user_pow_policy", get(get_user_pow_policy))
        .route("/foreign/deliver_email", post(deliver_email))
        .route("/foreign/verify_ip", post(verify_ip))
        .route(
            "/native/get_create_account_pow_policy",
            get(get_create_account_pow_policy),
        )
        .route("/native/create_account", post(create_account))
        .route("/native/get_emails", get(get_emails))
        .route("/auth/authenticate", post(authenticate))
        .route("/auth/refresh_access", post(refresh_access));

    let addr: SocketAddr = "0.0.0.0:8081".parse().unwrap();
    let tls_acceptor = tokio_rustls::TlsAcceptor::from(rustls_config);
    let tcp_listener = TcpListener::bind(&addr).await.unwrap();

    println!("HTTPS server listening on https://{addr}");

    loop {
        let tower_service = app.clone();
        let tls_acceptor = tls_acceptor.clone();

        let (cnx, addr) = tcp_listener.accept().await.unwrap();

        tokio::spawn(async move {
            let Ok(stream) = tls_acceptor.accept(cnx).await else {
                error!("error during tls handshake connection from {}", addr);
                return;
            };

            let stream = TokioIo::new(stream);

            let hyper_service =
                hyper::service::service_fn(move |mut request: Request<Incoming>| {
                    request.extensions_mut().insert(ConnectInfo(addr));
                    println!("{:?}", request.headers());
                    let auth_header = AuthorizationHeader::from_auth_header(
                        request.headers().get("Authorization"),
                    );
                    request.extensions_mut().insert(auth_header);
                    tower_service.clone().call(request)
                });

            let ret = hyper_util::server::conn::auto::Builder::new(TokioExecutor::new())
                .serve_connection_with_upgrades(stream, hyper_service)
                .await;

            if let Err(err) = ret {
                warn!("error serving connection from {}: {}", addr, err);
            }
        });
    }
}

async fn root() -> &'static str {
    "<h1>Hello, World!</h1>"
}

fn rustls_server_config(key: impl AsRef<Path>, cert: impl AsRef<Path>) -> Arc<ServerConfig> {
    let key = PrivateKeyDer::from_pem_file(key).unwrap();

    let certs = CertificateDer::pem_file_iter(cert)
        .unwrap()
        .map(|cert| cert.unwrap())
        .collect();

    let config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, key)
        .expect("bad certificate/key");

    Arc::new(config)
}
