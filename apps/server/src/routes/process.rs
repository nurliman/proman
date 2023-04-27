use std::sync::Arc;

use axum::{
  body::Body,
  extract::{Path, State},
  routing::get,
  Json, Router,
};
use mongodb::bson::oid::ObjectId;

use crate::{errors::AppError, models::process::Process, repository::process::ProcessRepository};

pub fn create_router() -> Router<Arc<ProcessRepository>, Body> {
  Router::new()
    .route("/processes", get(get_processes))
    .route("/processes/:id", get(get_process_by_id))
}

async fn get_processes(
  State(process_repository): State<Arc<ProcessRepository>>,
) -> Result<Json<Vec<Process>>, AppError> {
  let processes = process_repository.get_all().await?;
  Ok(Json(processes))
}

async fn get_process_by_id(
  State(process_repository): State<Arc<ProcessRepository>>,
  Path(id): Path<String>,
) -> Result<Json<Process>, AppError> {
  let object_id = ObjectId::parse_str(&id).expect("Invalid ObjectId");
  let process = process_repository.get_by_id(&object_id).await?;

  Ok(Json(process))
}
