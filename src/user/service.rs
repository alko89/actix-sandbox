use mongodb::{bson::doc, options::IndexOptions, Client, IndexModel};

use crate::config::env::MONGODB_DATABASE;
use crate::user::constants::COLL_NAME;
use crate::user::model::User;

/// Creates an index on the "username" field to force the values to be unique.
pub async fn create_username_index(client: &Client) {
  let options = IndexOptions::builder().unique(true).build();
  let model = IndexModel::builder()
      .keys(doc! { "username": 1 })
      .options(options)
      .build();
  client
      .database(&*MONGODB_DATABASE)
      .collection::<User>(COLL_NAME)
      .create_index(model, None)
      .await
      .expect("creating an index should succeed");
}
