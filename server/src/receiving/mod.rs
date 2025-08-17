mod auth_util;
mod routes;

use crate::config::config_file::CONFIG;
use crate::receiving::routes::auth::authenticate::authenticate;
use crate::receiving::routes::auth::check_auth::check_auth;
use crate::receiving::routes::auth::refresh_access::refresh_access;
use crate::receiving::routes::foreign::get_user_pow_policy_interserver::get_user_pow_policy_interserver;
use crate::receiving::routes::foreign::verify_ip::verify_ip;
use crate::receiving::routes::native::add_whitelist::add_whitelist;
use crate::receiving::routes::native::get_user_pow_policy::get_user_pow_policy;
use crate::receiving::routes::native::get_whitelist::get_whitelist;
use crate::receiving::routes::native::remove_whitelist::remove_whitelist;
use crate::receiving::routes::native::send_hmail::send_hmail;
use auth_util::auth_header::AuthorizationHeader;
use axum::extract::ConnectInfo;
use axum::routing::{delete, post};
use axum::{Router, extract::Request, routing::get};
use h_mail_interface::interface::routes::auth::authenticate::AUTH_AUTHENTICATE_PATH;
use h_mail_interface::interface::routes::auth::check_auth::AUTH_CHECK_AUTH_PATH;
use h_mail_interface::interface::routes::auth::refresh_access::AUTH_REFRESH_ACCESS_PATH;
use h_mail_interface::interface::routes::check_pow::CHECK_POW_PATH;
use h_mail_interface::interface::routes::foreign::deliver_hmail::FOREIGN_DELIVER_HMAIL_PATH;
use h_mail_interface::interface::routes::foreign::get_anonymous_user_pow_policy::FOREIGN_GET_ANONYMOUS_USER_POW_POLICY_PATH;
use h_mail_interface::interface::routes::foreign::get_user_pow_policy_interserver::FOREIGN_GET_USER_POW_POLICY_INTERSERVER_PATH;
use h_mail_interface::interface::routes::foreign::verify_ip::FOREIGN_VERIFY_IP_PATH;
use h_mail_interface::interface::routes::get_pow_token::GET_POW_TOKEN_PATH;
use h_mail_interface::interface::routes::native::add_whitelist::NATIVE_ADD_WHITELIST_PATH;
use h_mail_interface::interface::routes::native::create_account::NATIVE_CREATE_ACCOUNT_PATH;
use h_mail_interface::interface::routes::native::get_create_account_pow_policy::NATIVE_GET_CREATE_ACCOUNT_POW_POLICY_PATH;
use h_mail_interface::interface::routes::native::get_hmails::NATIVE_GET_HMAILS_PATH;
use h_mail_interface::interface::routes::native::get_user_pow_policy::NATIVE_GET_USER_POW_POLICY_PATH;
use h_mail_interface::interface::routes::native::get_whitelist::NATIVE_GET_WHITELIST_PATH;
use h_mail_interface::interface::routes::native::remove_whitelist::NATIVE_REMOVE_WHITELIST_PATH;
use h_mail_interface::interface::routes::native::send_hmail::NATIVE_SEND_HMAIL_PATH;
use h_mail_interface::interface::routes::{CHECK_ALIVE_PATH, CHECK_ALIVE_RESPONSE};
use hyper::body::Incoming;
use hyper_util::rt::{TokioExecutor, TokioIo};
use routes::check_pow::check_pow;
use routes::foreign::deliver_hmail::deliver_hmail;
use routes::foreign::get_anonymous_user_pow_policy::get_anonymous_user_pow_policy;
use routes::foreign::get_pow_token::get_pow_token;
use routes::native::create_account::create_account;
use routes::native::get_create_account_pow_policy::get_create_account_pow_policy;
use routes::native::get_hmails::get_hmails;
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
use tracing::{error, info, warn};

pub async fn recv_main_blocking() {
    info!("Starting listener");

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

    let app = Router::new()
        .route(CHECK_ALIVE_PATH, get(check_alive))
        .route(CHECK_POW_PATH, post(check_pow))
        .route(GET_POW_TOKEN_PATH, get(get_pow_token))
        .route(
            FOREIGN_GET_ANONYMOUS_USER_POW_POLICY_PATH,
            get(get_anonymous_user_pow_policy),
        )
        .route(FOREIGN_DELIVER_HMAIL_PATH, post(deliver_hmail))
        .route(FOREIGN_VERIFY_IP_PATH, post(verify_ip))
        .route(
            FOREIGN_GET_USER_POW_POLICY_INTERSERVER_PATH,
            post(get_user_pow_policy_interserver),
        )
        .route(
            NATIVE_GET_CREATE_ACCOUNT_POW_POLICY_PATH,
            get(get_create_account_pow_policy),
        )
        .route(NATIVE_CREATE_ACCOUNT_PATH, post(create_account))
        .route(NATIVE_GET_HMAILS_PATH, get(get_hmails))
        .route(NATIVE_SEND_HMAIL_PATH, post(send_hmail))
        .route(NATIVE_GET_USER_POW_POLICY_PATH, post(get_user_pow_policy))
        .route(NATIVE_ADD_WHITELIST_PATH, post(add_whitelist))
        .route(NATIVE_REMOVE_WHITELIST_PATH, delete(remove_whitelist))
        .route(NATIVE_GET_WHITELIST_PATH, get(get_whitelist))
        .route(AUTH_AUTHENTICATE_PATH, post(authenticate))
        .route(AUTH_REFRESH_ACCESS_PATH, post(refresh_access))
        .route(AUTH_CHECK_AUTH_PATH, get(check_auth));

    let addr: SocketAddr = format!("0.0.0.0:{}", CONFIG.port()).parse().unwrap();
    let tls_acceptor = tokio_rustls::TlsAcceptor::from(rustls_config);
    let tcp_listener = TcpListener::bind(&addr).await.unwrap();

    info!("HTTPS server listening on https://{addr}");

    loop {
        let tower_service = app.clone();
        let tls_acceptor = tls_acceptor.clone();

        let (cnx, addr) = tcp_listener.accept().await.unwrap();

        tokio::spawn(async move {
            let Ok(stream) = tls_acceptor.accept(cnx).await else {
                error!("Error during tls handshake connection from {}", addr);
                return;
            };

            let stream = TokioIo::new(stream);

            let hyper_service =
                hyper::service::service_fn(move |mut request: Request<Incoming>| {
                    request.extensions_mut().insert(ConnectInfo(addr));
                    // println!("{:?}", request.headers());
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

async fn check_alive() -> &'static str {
    CHECK_ALIVE_RESPONSE
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
