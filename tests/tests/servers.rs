#![allow(warnings)]
#![allow(clippy::all)]
use argon2::password_hash::SaltString;
use derive_new::new;
use h_mail_client::interface::routes::CHECK_ALIVE_PATH;
use h_mail_interface::interface::pow::PowPolicy;
use h_mail_interface::server_config::{MIN_SALT_BYTES, ServerConfig};
use h_mail_interface::shared::get_url_for_path;
use rand::{Rng, thread_rng};
use std::fs::File;
use std::path::Path;
use std::process::{Child, Command};
use std::time::{Duration, Instant};
use std::{fs, thread};
use tempdir::TempDir;

#[derive(new, Debug)]
pub struct Server {
    process: Child,
    port: u16,
    tmp_dir: TempDir,
}

impl Server {
    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn address(&self) -> String {
        format!("127.0.0.1:{}", self.port)
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        thread::sleep(Duration::from_millis(100));
        println!(
            "Killing server at port {} in dir {}",
            self.port,
            self.tmp_dir.path().display()
        );
        self.process.kill().unwrap();
    }
}

pub async fn wait_for_response<T: AsRef<str>>(address: T, timeout: Duration) {
    let start = Instant::now();
    while start.elapsed() < timeout {
        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap();

        if let Ok(r) = client
            .get(address.as_ref())
            .timeout(Duration::from_millis(500))
            .send()
            .await
        {
            println!(
                "Response received for {}: {:?}",
                address.as_ref(),
                r.text().await.unwrap()
            );
            return;
        };
    }
    panic!(
        "Server {} did not respond within {:?}",
        address.as_ref(),
        timeout
    );
}

pub fn generate_salt() -> SaltString {
    let mut salt = [0u8; MIN_SALT_BYTES];
    thread_rng().fill(&mut salt[..]);
    SaltString::encode_b64(&salt).unwrap()
}

pub async fn start_servers(count: usize, test_user: bool) -> Vec<Server> {
    if Path::new("server_logs").is_dir() {
        fs::remove_dir_all("server_logs").unwrap();
    }
    fs::create_dir("server_logs").unwrap();

    #[cfg(debug_assertions)]
    Command::new("cargo")
        .arg("build")
        .current_dir("../server")
        .status()
        .unwrap();
    #[cfg(not(debug_assertions))]
    Command::new("cargo")
        .arg("build")
        .arg("-r")
        .current_dir("../server")
        .status()
        .unwrap();

    #[cfg(debug_assertions)]
    let server_prog = {
        if cfg!(unix) {
            "../target/debug/h-mail-server"
        } else if cfg!(windows) {
            "../target/debug/h-mail-server.exe"
        } else {
            unreachable!()
        }
    };
    #[cfg(not(debug_assertions))]
    let server_prog = {
        if cfg!(unix) {
            "../target/release/h-mail-server"
        } else if cfg!(windows) {
            "../target/release/h-mail-server.exe"
        } else {
            unreachable!()
        }
    };

    let default_server_config = ServerConfig::default();

    let mut port = 8080;
    let s: Vec<Server> = (0..count)
        .map(|_| {
            port += 1;
            let tmp_dir = TempDir::new("h-mail-server-test").unwrap();

            let config_file = tmp_dir.path().join("config.json");
            let server_config = ServerConfig {
                domain: "127.0.0.1".to_string(),
                port,
                create_account_pow_burden: *default_server_config.create_account_pow_burden(),
                pow_token_expiry_ms: default_server_config.pow_token_expiry_ms(),
                pow_rsa_bits: default_server_config.pow_rsa_bits(),
                refresh_token_expiry_ms: default_server_config.refresh_token_expiry_ms(),
                access_token_expiry_ms: default_server_config.access_token_expiry_ms(),
                verify_ip_token_expiry_ms: default_server_config.verify_ip_token_expiry_ms(),
                default_user_pow_policy: PowPolicy::new(
                    10,
                    *default_server_config.default_user_pow_policy().accepted(),
                    *default_server_config.default_user_pow_policy().personal(),
                ),
            };
            fs::write(
                config_file,
                serde_json::to_string_pretty(&server_config).unwrap(),
            )
            .unwrap();

            let log = File::create(format!("server_logs/{port}.ans")).expect("failed to open log");
            let log_error =
                File::create(format!("server_logs/{port}_err.ans")).expect("failed to open log");

            println!(
                "Starting server at port {} in dir {}",
                port,
                tmp_dir.path().display()
            );

            let mut c = Command::new(fs::canonicalize(server_prog).unwrap());
            let base = c
                .arg("--no-rate-limit")
                .env("SECRET_SALT", generate_salt().to_string());

            if test_user {
                base.arg("--test-user");
            };

            Server::new(
                base.current_dir(tmp_dir.path())
                    .stdout(log)
                    .stderr(log_error)
                    .spawn()
                    .expect("Failed to start server"),
                port,
                tmp_dir,
            )
        })
        .collect();

    for server in &s {
        wait_for_response(
            get_url_for_path(server.address(), CHECK_ALIVE_PATH),
            Duration::from_secs(2),
        )
        .await;
    }

    s
}
