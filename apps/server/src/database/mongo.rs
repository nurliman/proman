use async_once::AsyncOnce;
use lazy_static::lazy_static;

use crate::config::env::ENV_CONFIG;

lazy_static! {
  pub static ref MONGODB_CONNECTION: AsyncOnce<mongodb::Database> = AsyncOnce::new(async {
    let mongodb_uri = ENV_CONFIG.mongodb_uri.as_str();
    let mongodb_name = ENV_CONFIG.mongodb_name.as_str();

    mongodb::Client::with_uri_str(mongodb_uri)
      .await
      .expect("Failed to initialize MongoDB connection")
      .database(mongodb_name)
  });
}
