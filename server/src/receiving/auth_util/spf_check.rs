use crate::config::args::ARGS;
use mail_auth::spf::verify::SpfParameters;
use mail_auth::{MessageAuthenticator, SpfResult};
use std::net::{IpAddr, SocketAddr};
use tracing::warn;

pub async fn spf_check(connect_info: SocketAddr, source_user: &str, source_domain: &str) -> bool {
    let is_ip = source_domain
        .split(':')
        .next()
        .unwrap()
        .parse::<IpAddr>()
        .is_ok();
    if is_ip {
        warn!(
            "Skipping SPF check as {} is an IP, not domain",
            source_domain
        );
        true
    } else if ARGS.no_spf() {
        true
    } else {
        let authenticator = MessageAuthenticator::new_google().unwrap();
        let sender = format!("{}@{}", &source_user, &source_domain);
        let result = authenticator
            .verify_spf(SpfParameters::verify_mail_from(
                connect_info.ip(),
                "",
                "",
                &sender,
            ))
            .await;

        matches!(result.result(), SpfResult::Pass)
    }
}
