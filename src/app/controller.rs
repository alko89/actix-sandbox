use actix_web::{get, HttpResponse, Responder};
use once_cell::sync::Lazy;
use serde::{Serialize};

#[derive(Serialize)]
pub struct Hello {
    pub name: String,
    pub uptime: u64,
    pub version: String,
}

const STARTUP_TIME: Lazy<std::time::SystemTime> = Lazy::new(|| std::time::SystemTime::now());

#[get("/")]
async fn hello() -> impl Responder {
  let startup_time = *STARTUP_TIME;
  let uptime = std::time::SystemTime::now()
      .duration_since(startup_time)
      .unwrap()
      .as_secs();
  let hello = Hello {
      name: env!("CARGO_PKG_NAME").to_string(),
      uptime,
      version: env!("CARGO_PKG_VERSION").to_string(),
  };
  HttpResponse::Ok().json(hello)
}
