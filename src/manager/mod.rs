mod running;
pub mod start_all;
pub mod stop_all;

pub type StopRunningFn = Box<dyn FnOnce() + Send + Sync>;
