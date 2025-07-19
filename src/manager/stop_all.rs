use std::fs;
use crate::running::log::log_types::LogSeverity::Info;
use crate::running::log::log_types::LogSource::Manager;
use crate::running::log::log;
use crate::manager::running::{FUNCTION_LOCK, LOG_FILE, RUNNING};

pub fn stop_all<T: AsRef<str>>(reason: T) -> ! {
    let _guard = FUNCTION_LOCK.lock().unwrap();
    

    if let Some(running) = RUNNING.get() {
        log::log(Manager, Info, "Stopping all services");
        for (service, stop_fn) in running {
            log::log(Manager, Info, format!("Stopping {service}"));
            stop_fn.lock().unwrap().take().unwrap()();
            log::log(Manager, Info, format!("Stopped {service}"));
        }

        println!("[MANAGER] All service stopped");
    } else {
        println!("[MANAGER] stop_all called before start_all")
    }

    println!("[MANAGER] Stop reason: {}", reason.as_ref());

    if let Some(log_file) = LOG_FILE.get() {
        if let Ok(contents) = fs::read_to_string(log_file) {
            println!("[MANAGER] Last lines of log file:\n{contents}")
        }
    }

    
    std::process::exit(0);
}