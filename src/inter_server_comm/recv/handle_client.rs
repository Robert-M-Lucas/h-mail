use crate::log::LogSeverity::{Error, Info};
use crate::log::LogSource::Receiver;
use crate::log::log;
use std::io::Read;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

pub fn handle_client(mut stream: TcpStream) {
    let Ok(peer) = stream.peer_addr() else {
        log(
            Receiver,
            Error,
            "Failed to get peer address of connecting client",
        );
        return;
    };

    let mut buf = [0u8; 512];

    loop {
        match stream.read(&mut buf) {
            Ok(0) => {
                log(Receiver, Info, format!("Connection closed by {}", peer));
                break;
            }
            Ok(n) => {
                let data = String::from_utf8_lossy(&buf[..n]);
                log(Receiver, Info, format!("Received from {}: {}", peer, data));
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                thread::sleep(Duration::from_millis(100));
            }
            Err(e) => {
                log(Receiver, Error, format!("Error on stream {}: {}", peer, e));
                break;
            }
        }
    }
}
