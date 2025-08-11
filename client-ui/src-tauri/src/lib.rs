use h_mail_client::communication::check_alive as c_check_alive;
use h_mail_client::communication::check_auth as c_check_auth;
use h_mail_client::interface::pow::{PowIters, PowToken};
use h_mail_client::reexports::{AnyhowError, BigUint};
use h_mail_client::{
    get_server_address, reauthenticate as c_reauthenticate, solve_pow_iter, AuthCredentials,
    HResult,
};
use h_mail_client::{set_server_address, AuthError};
use hhmmss::Hhmmss;
use num_format::{Locale, ToFormattedString};
use serde::Serialize;
use std::cmp::max;
use std::time::{Duration, Instant};
use tauri::{AppHandle, Emitter};
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

async fn solve_pow_monitor(
    app: AppHandle,
    pow_token: &PowToken,
    iters: PowIters,
    hash: BigUint,
) -> BigUint {
    let mut pow_iter = solve_pow_iter(&hash, pow_token.token(), iters);
    let mut i: PowIters = 0;
    app.emit(
        "pow-progress",
        format!(
            "POW Progress: {}/{}",
            i.to_formatted_string(&Locale::en),
            iters.to_formatted_string(&Locale::en)
        ),
    )
    .unwrap();
    let start = Instant::now();
    let mut last = Instant::now() - Duration::from_secs(2);
    let pow_result = loop {
        if Instant::now() - last > Duration::from_secs(1) {
            last = Instant::now();
            let time_per_iter = (Instant::now() - start) / max(i, 1);
            let estimated_remaining = time_per_iter * (iters - i);
            app.emit(
                "pow-progress",
                format!(
                    "POW Progress: {}/{} | {} remaining | {} elapsed",
                    i.to_formatted_string(&Locale::en),
                    iters.to_formatted_string(&Locale::en),
                    estimated_remaining.hhmmss(),
                    start.elapsed().hhmmss()
                ),
            )
            .unwrap();
        }
        i += 1;
        if let Some(p) = pow_iter.next_iter() {
            break p;
        }
    };
    app.emit("pow-progress", "".to_string()).unwrap();
    pow_result
}

async fn create_account_inner(
    app: AppHandle,
    username: String,
    password: String,
) -> HResult<String> {
    todo!()
    // let create_account_request = CreateAccountPackage::new(username.clone(), password.clone());
    // let pow_token = get_pow_token(get_server_address().await?).await?;
    // let pow_token = pow_token.decode()?;
    // let pow_policy = get_create_account_pow_policy().await?;
    // let iters = *pow_policy.required();
    // let pow_result =
    //     solve_pow_monitor(app, &pow_token, iters, create_account_request.pow_hash()).await;
    //
    // let cr = c_create_account(&CreateAccountRequest::new(
    //     create_account_request,
    //     iters,
    //     BigUintField::new(pow_token.token()),
    //     BigUintField::new(&pow_result),
    // ))
    // .await?;
    // match cr {
    //     CreateAccountResponse::Success => {}
    //     CreateAccountResponse::BadUsername => {
    //         bail!("Bad username");
    //     }
    //     CreateAccountResponse::UsernameInUse => {
    //         bail!("Username in use");
    //     }
    //     CreateAccountResponse::BadPassword => {
    //         bail!("Bad password");
    //     }
    //     CreateAccountResponse::DoesNotMeetPolicy(_) => {
    //         bail!("Doesn't meet policy");
    //     }
    //     CreateAccountResponse::PowFailure(_) => {
    //         bail!("Pow failure");
    //     }
    // };
    // c_reauthenticate(AuthCredentials::new(username.clone(), password)).await?;
    // Ok(username)
}

#[tauri::command]
async fn create_account(
    app: AppHandle,
    username: String,
    password: String,
) -> InterfaceResult<String> {
    create_account_inner(app, username, password).await.into()
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
        .setup(|_app| {
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
