use actix_web::{get, post, web, HttpResponse};
use mongodb::{bson::doc, Client, Collection};

use crate::user::model::User;

const DB_NAME: &str = "myApp";
const COLL_NAME: &str = "users";

/// Adds a new user to the "users" collection in the database.
#[post("/user")]
async fn add_user(client: web::Data<Client>, body: web::Json<User>) -> HttpResponse {
    let collection = client.database(DB_NAME).collection(COLL_NAME);
    let result = collection.insert_one(body.into_inner(), None).await;
    match result {
        Ok(_) => HttpResponse::Ok().json("user added"),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

/// Gets the user with the supplied username.
#[get("/user/{username}")]
async fn get_user(client: web::Data<Client>, username: web::Path<String>) -> HttpResponse {
    let username = username.into_inner();
    let collection: Collection<User> = client.database(DB_NAME).collection(COLL_NAME);
    match collection
        .find_one(doc! { "username": &username }, None)
        .await
    {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => {
            HttpResponse::NotFound().json(format!("No user found with username {username}"))
        }
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}
