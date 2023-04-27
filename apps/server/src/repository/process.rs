use anyhow::Result;

use mongodb::bson::{doc, oid::ObjectId};
use tokio_stream::StreamExt;

use crate::models::process::Process;

pub struct ProcessRepository {
  processes_collection: mongodb::Collection<Process>,
}

impl ProcessRepository {
  pub fn new(processes_collection: mongodb::Collection<Process>) -> Self {
    Self {
      processes_collection,
    }
  }

  pub async fn create(&self, process: &Process) -> Result<ObjectId> {
    let result = self
      .processes_collection
      .insert_one(process.clone(), None)
      .await?;
    Ok(result.inserted_id.as_object_id().unwrap().to_owned())
  }

  pub async fn get_by_id(&self, id: &ObjectId) -> Result<Process> {
    let result = self
      .processes_collection
      .find_one(doc! {"_id": id.clone()}, None)
      .await?;

    match result {
      Some(process) => Ok(process),
      None => Err(anyhow::Error::msg("Process not found")),
    }
  }

  pub async fn get_all(&self) -> Result<Vec<Process>> {
    let mut cursor = self.processes_collection.find(doc! {}, None).await?;
    let mut processes = Vec::new();

    while let Some(document) = cursor.try_next().await? {
      processes.push(document);
    }

    Ok(processes)
  }

  pub async fn update(&self, id: &ObjectId, process: &Process) -> Result<()> {
    let result = self
      .processes_collection
      .replace_one(doc! {"_id": id.clone()}, process.clone(), None)
      .await?;
    // if result.matched_count == 0 {
    //   return Err(mongodb::error::Error::from(
    //     mongodb::error::ErrorKind::NoDocuments,
    //   ));
    // }
    Ok(())
  }

  pub async fn delete(&self, id: &ObjectId) -> Result<()> {
    let result = self
      .processes_collection
      .delete_one(doc! {"_id": id.clone()}, None)
      .await?;
    // if result.deleted_count == 0 {
    //   return Err(mongodb::error::Error::from(
    //     mongodb::error::ErrorKind::NoDocuments,
    //   ));
    // }
    Ok(())
  }
}
