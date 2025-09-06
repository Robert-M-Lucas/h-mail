use h_mail_client::communication::check_alive;
use h_mail_client::set_server_address;

#[tokio::main]
async fn main() {
    set_server_address("127.0.0.1:8081").await;
    println!("{:?}", check_alive().await)
    // h_mail_client::test()
}
