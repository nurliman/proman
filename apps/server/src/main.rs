use std::{net::SocketAddr, str::FromStr};

use axum::{routing::get, Router};
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;
use serde::Deserialize;

fn default_host() -> String {
  "127.0.0.1".to_string()
}

fn default_port() -> u16 {
  3000
}

fn default_db_url() -> String {
  "sqlite://data.db?mode=rwc".to_string()
}

#[derive(Deserialize, Debug)]
struct EnvVar {
  #[serde(default = "default_host")]
  server_host: String,

  #[serde(default = "default_port")]
  server_port: u16,

  #[serde(default = "default_db_url")]
  db_url: String,
}

#[tokio::main]
async fn main() {
  dotenvy::dotenv().ok();

  let env_var: EnvVar = envy::from_env().unwrap();

  let db_conn = Database::connect(env_var.db_url.as_str())
    .await
    .expect("Database connection failed");

  Migrator::up(&db_conn, None).await.unwrap();

  let app = Router::new().route("/", get(handler));

  let server_url = format!("{}:{}", env_var.server_host, env_var.server_port);
  let server_addr = SocketAddr::from_str(&server_url).unwrap();

  println!("Listening on {}", server_addr);

  axum::Server::bind(&server_addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
}

async fn handler() -> &'static str {
  "Hello, world!2222"
}
