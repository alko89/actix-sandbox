use once_cell::sync::Lazy;

pub static STARTUP_TIME: Lazy<std::time::SystemTime> = Lazy::new(|| std::time::SystemTime::now());
