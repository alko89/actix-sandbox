use actix_web::{web, App, HttpServer, middleware::Logger};
use mongodb::Client;

mod config;
use config::constants::{STARTUP_TIME};
use config::environment::ENV;

mod app;
use app::controller::{hello};

mod user;
use user::service::{create_username_index};
use user::controller::{add_user, get_user};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // force the lazy static to be initialized
    once_cell::sync::Lazy::force(&STARTUP_TIME);

    let uri = &ENV.mongodb_uri;

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
