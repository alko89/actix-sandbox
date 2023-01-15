use once_cell::sync::Lazy;

pub static MONGODB_URI: Lazy<String> = Lazy::new(|| std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into()));
pub static MONGODB_DATABASE: Lazy<String> = Lazy::new(|| std::env::var("MONGODB_DATABASE").unwrap_or_else(|_| "actix".into()));
