mod config;
mod database;
mod errors;
mod logger;
mod models;
mod repository;
mod routes;

use axum::Router;
use std::{net::SocketAddr, str::FromStr, sync::Arc};

use crate::{
  config::env::ENV_CONFIG, database::mongo::MONGODB_CONNECTION, models::process::Process,
  repository::process::ProcessRepository,
};

#[tokio::main]
async fn main() {
  logger::init_logger();

  let mongodb_conn = MONGODB_CONNECTION.get().await;

  let process_collection = mongodb_conn.collection::<Process>("processes");

  let process_repository = Arc::new(ProcessRepository::new(process_collection));

  let app = Router::new()
    .merge(routes::process::create_router())
    .with_state(process_repository)
    .layer(logger::create_logger_middleware());

  let server_url = format!("{}:{}", ENV_CONFIG.server_host, ENV_CONFIG.server_port);
  let server_address = SocketAddr::from_str(&server_url).unwrap();

  tracing::debug!("Listening on {}", server_address);

  axum::Server::bind(&server_address)
    .serve(app.into_make_service())
    .await
    .unwrap();
}
