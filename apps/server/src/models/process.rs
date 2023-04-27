use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, time::SystemTime};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Process {
  #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
  pub id: Option<ObjectId>,
  pub pid: u32,
  pub name: String,
  pub status: ProcessStatus,
  pub env: HashMap<String, String>,
  pub created_at: SystemTime,
  pub updated_at: SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProcessStatus {
  Online,
  Stopped,
  Restarting,
  Errored,
}
