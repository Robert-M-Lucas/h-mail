use std::io::{Write, stdin, stdout};
use tokio::io::AsyncBufReadExt;

pub fn read_line() -> String {
    stdout().flush().unwrap();
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
