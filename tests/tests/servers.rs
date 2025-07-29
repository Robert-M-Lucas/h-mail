use std::fs;
use std::process::{Child, Command, Stdio};
use derive_new::new;
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

pub fn start_servers(count: usize) -> Vec<Server> {
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
    (0..count)
        .map(|_| {
            port += 1;
            let tmp_dir = TempDir::new("h-mail-server-test").unwrap();

            println!(
                "Starting server at port {} in dir {}",
                port,
                tmp_dir.path().display()
            );

            Server::new(
                Command::new(fs::canonicalize(server_prog).unwrap())
                    .arg("--port")
                    .arg(format!("{port}"))
                    .current_dir(tmp_dir.path())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                    .expect("Failed to start server"),
                port,
                tmp_dir,
            )
        })
        .collect()
}