use crate::APP_HANDLE;
use derive_getters::Dissolve;
use derive_new::new;
use h_mail_client::interface::fields::big_uint::BigUintField;
use h_mail_client::interface::pow::{PowHash, PowIters, PowResult};
use h_mail_client::interface::routes::native::create_account::CreateAccountPackage;
use h_mail_client::reexports::rsa::traits::PublicKeyParts;
use h_mail_client::reexports::rsa::RsaPrivateKey;
use h_mail_client::reexports::BigUint;
use h_mail_client::{solve_pow_iter, ROUGH_POW_ITER_PER_SECOND};
use hhmmss::Hhmmss;
use num_format::{Locale, ToFormattedString};
use once_cell::sync::Lazy;
use rand::rngs::OsRng;
use std::cmp::max;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use tauri::Emitter;
use tokio::sync::oneshot;
use tracing::debug;

#[derive(new, Dissolve)]
pub struct PowSolveRequest {
    token: BigUint,
    iters: PowIters,
    hash: BigUint,
}

#[derive(Dissolve)]
struct Request {
    data: PowSolveRequest,
    resp_tx: oneshot::Sender<Option<BigUint>>,
}

static CANCEL_POW: AtomicBool = AtomicBool::new(false);

fn solve_pow_monitor(pow_token: &BigUint, iters: PowIters, hash: &BigUint) -> Option<BigUint> {
    let app = APP_HANDLE.get().unwrap().clone();

    let mut pow_iter = solve_pow_iter(hash, pow_token, iters);
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
        if CANCEL_POW.swap(false, Ordering::AcqRel) {
            app.emit("pow-progress", "".to_string()).unwrap();
            return None;
        }

        if Instant::now() - last > Duration::from_millis(500) {
            last = Instant::now();
            let time_per_iter = (Instant::now() - start) / max(i, 1);
            let estimated_remaining = time_per_iter * (iters - i);
            app.emit(
                "pow-progress",
                format!(
                    "{}#{}${} remaining | {} elapsed",
                    i,
                    iters,
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
    Some(pow_result)
}

static WORKER: Lazy<Mutex<mpsc::Sender<Request>>> = Lazy::new(|| {
    let (tx, rx) = mpsc::channel::<Request>();

    thread::spawn(move || {
        for req in rx {
            let (data, resp_tx): (PowSolveRequest, oneshot::Sender<Option<BigUint>>) =
                req.dissolve();
            let (token, iters, hash) = data.dissolve();

            // Flag set too early
            CANCEL_POW.store(false, Ordering::Release);
            let result = solve_pow_monitor(&token, iters, &hash);

            let _ = resp_tx.send(result);
        }
    });

    Mutex::new(tx)
});

pub async fn queue_solve_pow_result(
    token: &BigUint,
    iters: PowIters,
    hash: &BigUint,
) -> Option<PowResult> {
    let result = queue_solve_pow(PowSolveRequest::new(token.clone(), iters, hash.clone())).await;

    Some(PowResult::new(
        iters,
        BigUintField::new(token),
        BigUintField::new(&result?),
    ))
}

pub async fn queue_solve_pow(data: PowSolveRequest) -> Option<BigUint> {
    let (resp_tx, resp_rx) = oneshot::channel();

    let req = Request { data, resp_tx };

    WORKER.lock().unwrap().send(req).expect("Worker queue full");

    resp_rx.await.expect("Worker dropped the result")
}

#[tauri::command]
pub async fn estimate_performance() -> f64 {
    debug!("estimate_performance");

    let hash = CreateAccountPackage::new("alpha".to_string(), "bravo".to_string()).pow_hash();
    let mut rng = OsRng; // rand@0.8
    let priv_key = RsaPrivateKey::new(&mut rng, 2048).unwrap();
    let pow_token = priv_key.n().clone();
    let mut pow_iter = solve_pow_iter(&hash, &pow_token, PowIters::MAX);

    let mut i: PowIters = 0;
    let start = Instant::now();

    const DURATION: Duration = Duration::from_secs(5);

    while Instant::now() - start < DURATION {
        if let Some(_p) = pow_iter.next_iter() {
            panic!()
        }
        i += 1
    }

    if i == 0 {
        ROUGH_POW_ITER_PER_SECOND as f64
    } else {
        let v = i as f64 / start.elapsed().as_secs_f64();
        let to_write = format!("{}", v as u64);
        tokio::fs::write("performance_estimate", to_write)
            .await
            .ok();
        v
    }
}

#[tauri::command]
pub async fn load_estimate() -> Option<f64> {
    debug!("load_estimate");

    let estimate = tokio::fs::read_to_string("performance_estimate")
        .await
        .ok()?;
    let estimate = estimate.parse::<u64>().ok()?;
    Some(estimate as f64)
}

#[tauri::command]
pub async fn cancel_current_pow() {
    debug!("cancel_current_pow");
    CANCEL_POW.store(true, Ordering::Release)
}
