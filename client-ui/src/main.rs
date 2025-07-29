use h_mail_client::{reauthenticate, set_server_address, AuthCredentials};
use h_mail_client::communication::ping_server;

#[tokio::main]
async fn main() {
    set_server_address("localhost:8081").await;
    let r = ping_server().await;
    println!("{:?}", r);
    // reauthenticate(AuthCredentials::new("test".to_string(), "test".to_string())).await.unwrap();
    
}
