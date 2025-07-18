use crate::manager::StopRunningFn;
use std::sync::{Mutex, OnceLock};

pub static FUNCTION_LOCK: once_cell::sync::Lazy<Mutex<()>> =
    once_cell::sync::Lazy::new(|| Mutex::new(()));

pub static RUNNING: OnceLock<Vec<(&'static str, Mutex<Option<StopRunningFn>>)>> = OnceLock::new();
