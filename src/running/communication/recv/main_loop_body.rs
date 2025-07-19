use crate::running::database::conn::DatabaseRef;
use crate::running::communication::recv::handle_client::handle_client;
use crate::running::log::log_types::LogSeverity::Error;
use crate::running::log::log_types::LogSource::Receiver;
use crate::running::log::log;
use std::net::TcpListener;
use std::thread;
use std::time::Duration;

pub fn main_loop_body(db: &DatabaseRef, listener: &TcpListener) {
    match listener.accept() {
        Ok((stream, _)) => {
            if stream.set_nonblocking(true).is_err() {
                log::log(
                    Receiver,
                    Error,
                    "Failed to set client stream to non-blocking",
                );
            }
            thread::spawn(|| handle_client(stream));
        }
        Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
            thread::sleep(Duration::from_millis(50));
        }
        Err(e) => {
            eprintln!("Listener error: {e}");
            thread::sleep(Duration::from_millis(50));
        }
    }
}
