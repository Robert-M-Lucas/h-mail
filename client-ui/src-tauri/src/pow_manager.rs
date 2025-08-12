use crate::APP_HANDLE;
use derive_getters::Dissolve;
use derive_new::new;
use h_mail_client::interface::pow::PowIters;
use h_mail_client::reexports::BigUint;
use h_mail_client::solve_pow_iter;
use hhmmss::Hhmmss;
use num_format::{Locale, ToFormattedString};
use once_cell::sync::Lazy;
use std::cmp::max;
use std::sync::{mpsc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use tauri::Emitter;
use tokio::sync::oneshot;

#[derive(new, Dissolve)]
pub struct PowSolveRequest {
    token: BigUint,
    iters: PowIters,
    hash: BigUint,
}

#[derive(Dissolve)]
struct Request {
    data: PowSolveRequest,
    resp_tx: oneshot::Sender<BigUint>,
}

fn solve_pow_monitor(pow_token: &BigUint, iters: PowIters, hash: &BigUint) -> BigUint {
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
    pow_result
}

static WORKER: Lazy<Mutex<mpsc::Sender<Request>>> = Lazy::new(|| {
    let (tx, rx) = mpsc::channel::<Request>();

    thread::spawn(move || {
        for req in rx {
            let (data, resp_tx): (PowSolveRequest, oneshot::Sender<BigUint>) = req.dissolve();
            let (token, iters, hash) = data.dissolve();

            let result = solve_pow_monitor(&token, iters, &hash);

            let _ = resp_tx.send(result);
        }
    });

    Mutex::new(tx)
});

pub async fn queue_solve_pow(data: PowSolveRequest) -> BigUint {
    let (resp_tx, resp_rx) = oneshot::channel();

    let req = Request { data, resp_tx };

    WORKER.lock().unwrap().send(req).expect("Worker queue full");

    resp_rx.await.expect("Worker dropped the result")
}
