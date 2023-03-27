use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(Project::Table)
          .if_not_exists()
          .col(
            ColumnDef::new(Project::Id)
              .uuid()
              .not_null()
              .primary_key(),
          )
          .col(ColumnDef::new(Project::Name).string().not_null())
          .col(
            ColumnDef::new(Project::Description)
              .string()
              .not_null()
              .default("".to_owned()),
          )
          .col(
            ColumnDef::new(Project::CreatedAt)
              .timestamp()
              .not_null()
              .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
          )
          .col(
            ColumnDef::new(Project::UpdatedAt)
              .timestamp()
              .not_null()
              .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
          )
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().table(Project::Table).to_owned())
      .await
  }
}

#[derive(Iden)]
#[iden(rename = "projects")]
enum Project {
  Table,
  Id,
  Name,
  Description,
  CreatedAt,
  UpdatedAt,
}
