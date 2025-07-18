use crate::log::LogSeverity::Info;
use crate::log::LogSource::Manager;
use crate::log::log;
use crate::manager::running::{FUNCTION_LOCK, RUNNING};

pub fn stop_all() -> ! {
    let _guard = FUNCTION_LOCK.lock().unwrap();
    log(Manager, Info, "Stopping all services");

    let Some(running) = RUNNING.get() else {
        panic!("stop_all called before start_all")
    };

    for (service, stop_fn) in running {
        log(Manager, Info, format!("Stopping {service}"));
        stop_fn.lock().unwrap().take().unwrap()();
        log(Manager, Info, format!("Stopped {service}"));
    }

    println!("[MANAGER] All service stopped. Exiting process.");
    std::process::exit(0);
}
