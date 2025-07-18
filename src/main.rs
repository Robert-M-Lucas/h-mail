use manager::start_all::start_all;

mod client_comm;
mod database;
mod inter_server_comm;
mod log;
mod manager;
mod pow;
mod terminal;

fn main() {
    start_all();
}
