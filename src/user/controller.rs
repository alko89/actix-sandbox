use actix_web::{get, post, web, HttpResponse};
use mongodb::{bson::doc, Client, Collection};

use crate::config::environment::ENV;
use crate::config::error::Error;

use crate::user::constants::COLL_NAME;
use crate::user::model::User;

/// Adds a new user to the "users" collection in the database.
#[post("/")]
async fn add_user(client: web::Data<Client>, body: web::Json<User>) -> HttpResponse {
    let collection = client.database(&ENV.mongodb_database).collection(COLL_NAME);
    let result = collection.insert_one(body.into_inner(), None).await;
    match result {
        Ok(_) => HttpResponse::Ok().json("user added"),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

/// Gets the user with the supplied username.
#[get("/{username}")]
async fn get_user(client: web::Data<Client>, username: web::Path<String>) -> HttpResponse {
    let username = username.into_inner();
    let collection: Collection<User> = client.database(&ENV.mongodb_database).collection(COLL_NAME);
    match collection
        .find_one(doc! { "username": &username }, None)
        .await
    {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => {
            HttpResponse::NotFound().json(Error {
                msg: format!("No user found with username {username}"),
                status: 404,
            })
        }
        Err(err) => HttpResponse::InternalServerError().json(Error {
            msg: err.to_string(),
            status: 500,
        }),
    }
}
