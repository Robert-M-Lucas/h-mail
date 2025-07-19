use std::fs::File;
use std::path::PathBuf;
use crate::manager::StopRunningFn;
use std::sync::{Mutex, OnceLock};
use once_cell::sync::Lazy;

pub static FUNCTION_LOCK: Lazy<Mutex<()>> =
    Lazy::new(|| Mutex::new(()));

pub static LOG_FILE: OnceLock<PathBuf> = OnceLock::new();
pub static RUNNING: OnceLock<Vec<(&'static str, Mutex<Option<StopRunningFn>>)>> = OnceLock::new();
