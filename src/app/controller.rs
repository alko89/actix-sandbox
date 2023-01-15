use actix_web::{get, HttpResponse, Responder};
use serde::{Serialize};

use crate::config::constants::{STARTUP_TIME};

#[derive(Serialize)]
pub struct Hello {
    pub name: String,
    pub uptime: u64,
    pub version: String,
}

#[get("/")]
async fn hello() -> impl Responder {
  let uptime = std::time::SystemTime::now()
      .duration_since(*STARTUP_TIME)
      .unwrap()
      .as_secs();
  let hello = Hello {
      name: env!("CARGO_PKG_NAME").to_string(),
      uptime,
      version: env!("CARGO_PKG_VERSION").to_string(),
  };
  HttpResponse::Ok().json(hello)
}
