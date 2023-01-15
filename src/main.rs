use std::{
    fmt::{Display, Formatter, Result as FmtResult},
};

use actix_web::{web, App, HttpServer, middleware::Logger, HttpResponse, http::StatusCode, ResponseError};
use env_logger::Env;
use mongodb::{bson::doc, Client};
use serde::Serialize;
use serde_json::{json, to_string_pretty};

mod config;
use config::constants::{STARTUP_TIME};
use config::env::*;

mod app;
use app::controller::{hello};

mod user;
use user::service::{create_username_index};
use user::controller::{add_user, get_user};

#[derive(Debug, Serialize)]
struct Error {
    msg: String,
    status: u16,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", to_string_pretty(self).unwrap())
    }
}

impl ResponseError for Error {
    // builds the actual response to send back when an error occurs
    fn error_response(&self) -> HttpResponse {
        let err_json = json!({ "error": self.msg });
        HttpResponse::build(StatusCode::from_u16(self.status).unwrap()).json(err_json)
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // force the lazy static to be initialized
    once_cell::sync::Lazy::force(&STARTUP_TIME);

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let uri = &*MONGODB_URI;

    let client = Client::with_uri_str(uri).await.expect("failed to connect");
    create_username_index(&client).await;

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(client.clone()))
            .service(hello)
            .service(web::scope("/user")
                .service(add_user)
                .service(get_user)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
