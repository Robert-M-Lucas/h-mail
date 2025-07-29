use derive_new::new;
use std::fs::File;
use std::path::Path;
use std::process::{Child, Command};
use std::time::{Duration, Instant};
use std::fs;
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
        format!("localhost:{}", self.port)
    }
}

impl Drop for Server {
    fn drop(&mut self) {
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

        if client
            .get(address.as_ref())
            .timeout(Duration::from_millis(500))
            .send()
            .await
            .is_ok()
        {
            return;
        };
    }
    panic!(
        "Server {} did not respond within {:?}",
        address.as_ref(),
        timeout
    );
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
    let server_prog = "../target/debug/h-mail-server";
    #[cfg(not(debug_assertions))]
    let server_prog = "../target/release/h-mail-server";

    let mut port = 8080;
    let s: Vec<Server> = (0..count)
        .map(|_| {
            port += 1;
            let tmp_dir = TempDir::new("h-mail-server-test").unwrap();

            let log = File::create(format!("server_logs/{port}.txt")).expect("failed to open log");
            let log_error =
                File::create(format!("server_logs/{port}_err.txt")).expect("failed to open log");

            println!(
                "Starting server at port {} in dir {}",
                port,
                tmp_dir.path().display()
            );

            let mut c = Command::new(fs::canonicalize(server_prog).unwrap());
            let base = c.arg("--port").arg(format!("{port}"));

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
            format!("https://localhost:{}/", server.port()),
            Duration::from_secs(2),
        )
        .await;
    }

    s
}
