use h_mail_client::communication::{get_emails, ping_server};
use h_mail_client::interface::routes::native::get_emails::GetEmailsRequest;
use h_mail_client::{AuthCredentials, AuthError, reauthenticate, set_server_address};

#[tokio::main]
async fn main() {
    set_server_address("localhost:8081").await;
    ping_server().await.unwrap();

    let r = match get_emails(&GetEmailsRequest::new(-1)).await {
        Ok(v) => v,
        Err(AuthError::RequireReauth) => {
            reauthenticate(AuthCredentials::new("test".to_string(), "test".to_string()))
                .await
                .unwrap();
            get_emails(&GetEmailsRequest::new(-1)).await.unwrap()
        }
        Err(e) => {
            panic!("{:?}", e)
        }
    };

    println!("{r:?}")
}
