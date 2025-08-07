use h_mail_client::communication::check_auth as c_check_auth;
use h_mail_client::communication::create_account as c_create_account;
use h_mail_client::communication::{check_alive as c_check_alive, get_create_account_pow_policy, get_pow_token};
use h_mail_client::interface::fields::big_uint::BigUintField;
use h_mail_client::interface::pow::PowHash;
use h_mail_client::interface::routes::native::create_account::{CreateAccountPackage, CreateAccountRequest, CreateAccountResponse};
use h_mail_client::{get_server_address, reauthenticate as c_reauthenticate, solve_pow, AuthCredentials, HResult};
use h_mail_client::{set_server_address, AnyhowError, AuthError};
use h_mail_client::anyhow::bail;
use serde::Serialize;
use tokio::fs;

#[tauri::command]
async fn check_alive() -> String {
    if c_check_alive().await.is_ok() {
        "Alive".to_string()
    } else {
        "Not Alive".to_string()
    }
}

#[derive(Serialize)]
enum InterfaceAuthResult<T> {
    Unauthorized,
    Success(T),
}

#[derive(Serialize)]
enum InterfaceResult<T> {
    Ok(T),
    Err(String),
}

impl<T> InterfaceResult<T> {
    pub fn from_error(e: AnyhowError) -> Self {
        InterfaceResult::Err(format!("{e}"))
    }
}

impl<T> From<HResult<T>> for InterfaceResult<T> {
    fn from(value: HResult<T>) -> Self {
        match value {
            Ok(v) => InterfaceResult::Ok(v),
            Err(e) => Self::from_error(e),
        }
    }
}

#[tauri::command]
async fn check_auth() -> InterfaceResult<InterfaceAuthResult<String>> {
    match c_check_auth().await {
        Ok(v) => InterfaceResult::Ok(InterfaceAuthResult::Success(v.username().clone())),
        Err(e) => match e {
            AuthError::RequireReauth => InterfaceResult::Ok(InterfaceAuthResult::Unauthorized),
            AuthError::Other(e) => InterfaceResult::from_error(e),
        },
    }
}

#[tauri::command]
async fn reauthenticate(username: String, password: String) -> InterfaceResult<String> {
    match c_reauthenticate(AuthCredentials::new(username.clone(), password.to_string())).await {
        Ok(_) => InterfaceResult::Ok(username),
        Err(e) => InterfaceResult::from_error(e),
    }
}

async fn create_account_inner(username: String, password: String) -> HResult<String> {
    let create_account_request = CreateAccountPackage::new(username.clone(), password.clone());
    let pow_token = get_pow_token(get_server_address().await?).await?;
    let pow_token = pow_token.decode()?;
    let pow_policy = get_create_account_pow_policy().await?;
    let iters = *pow_policy.required();
    let pow_result = solve_pow(&create_account_request.pow_hash(), pow_token.token(), iters);
    let cr = c_create_account(
        &CreateAccountRequest::new(
            create_account_request,
            iters,
            BigUintField::new(pow_token.token()),
            BigUintField::new(&pow_result)
        )
    ).await?;
    println!("{:?}", cr);
    match cr {
        CreateAccountResponse::Success => {}
        CreateAccountResponse::BadUsername => {
            bail!("Bad username");
        }
        CreateAccountResponse::UsernameInUse => {
            bail!("Username in use");
        }
        CreateAccountResponse::BadPassword => {
            bail!("Bad password");
        }
        CreateAccountResponse::DoesNotMeetPolicy(_) => {
            bail!("Doesn't meet policy");
        }
        CreateAccountResponse::PowFailure(_) => {
            bail!("Pow failure");
        }
    };
    c_reauthenticate(AuthCredentials::new(username.clone(), password)).await?;
    Ok(username)
}

#[tauri::command]
async fn create_account(username: String, password: String) -> InterfaceResult<String> {
    create_account_inner(username, password).await.into()
}

#[tauri::command]
async fn set_server(server: String) {
    set_server_address(&server).await;
    fs::write("server_address", server).await.unwrap();
}

#[tauri::command]
async fn get_server() -> InterfaceResult<String> {
    get_server_address().await.into()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            tauri::async_runtime::block_on(async {
                if let Ok(v) = fs::read_to_string("server_address").await {
                    set_server_address(v).await;
                }
            });

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            set_server,
            get_server,
            check_alive,
            check_auth,
            reauthenticate,
            create_account
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
