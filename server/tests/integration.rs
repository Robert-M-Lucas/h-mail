use derive_new::new;
use itertools::Itertools;
use std::fs;
use std::process::{Child, Command, Stdio};
use std::thread::sleep;
use std::time::Duration;
use tempdir::TempDir;

#[derive(new, Debug)]
struct Server {
    process: Child,
    port: u16,
    tmp_dir: TempDir,
}

impl Server {
    pub fn port(&self) -> u16 {
        self.port
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

fn start_servers(count: usize) -> Vec<Server> {
    #[cfg(debug_assertions)]
    Command::new("cargo").arg("build").status().unwrap();
    #[cfg(not(debug_assertions))]
    Command::new("cargo")
        .arg("build")
        .arg("-r")
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
        .collect_vec()
}

#[test]
fn test() {
    let servers = start_servers(2);
    println!("{servers:?}");
    sleep(Duration::from_secs(3));
    assert!(false)
}
