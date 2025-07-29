use h_mail_client::{AuthCredentials, AuthError, reauthenticate, set_server_address};
use h_mail_client::communication::get_emails;
use h_mail_client::interface::routes::native::get_emails::GetEmailsRequest;
use crate::servers::start_servers;

mod servers;

#[tokio::test]
async fn test() {
    let server = start_servers(1, true).await;
    
    set_server_address(server[0].address()).await;

    let r = GetEmailsRequest::new(-1);

    let e = get_emails(&r).await;

    let v = match e {
        Ok(v) => v,
        Err(AuthError::RequireReauth) => {
            reauthenticate(AuthCredentials::new("test".to_string(), "test".to_string()))
                .await
                .unwrap();
            get_emails(&r).await.unwrap()
        }
        Err(AuthError::Other(e)) => {
            panic!("{e:?}");
        }
    };

    println!("{v:?}");
}
