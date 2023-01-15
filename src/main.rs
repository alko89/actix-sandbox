use std::{
    fmt::{Display, Formatter, Result as FmtResult},
};

use actix_web::{web, App, HttpServer, middleware::Logger, HttpResponse, http::StatusCode, ResponseError};
use env_logger::Env;
use mongodb::{bson::doc, options::IndexOptions, Client, IndexModel};
use serde::Serialize;
use serde_json::{json, to_string_pretty};

mod app;
use app::controller::{hello};

mod user;
use user::model::User;
use user::controller::{add_user, get_user};

const DB_NAME: &str = "myApp";
const COLL_NAME: &str = "users";

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

/// Creates an index on the "username" field to force the values to be unique.
async fn create_username_index(client: &Client) {
    let options = IndexOptions::builder().unique(true).build();
    let model = IndexModel::builder()
        .keys(doc! { "username": 1 })
        .options(options)
        .build();
    client
        .database(DB_NAME)
        .collection::<User>(COLL_NAME)
        .create_index(model, None)
        .await
        .expect("creating an index should succeed");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());

    let client = Client::with_uri_str(uri).await.expect("failed to connect");
    create_username_index(&client).await;

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(client.clone()))
            .service(hello)
            .service(add_user)
            .service(get_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
