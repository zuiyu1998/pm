use pm_sea_orm::task::{TaskColumn, TaskEntity};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TaskEntity)
                    .if_not_exists()
                    .col(pk_auto(TaskColumn::Id))
                    .col(string(TaskColumn::Title))
                    .col(big_integer(TaskColumn::Hash).unique_key())
                    .col(boolean(TaskColumn::Completed).default(false))
                    .col(string_null(TaskColumn::Goal))
                    .col(string_null(TaskColumn::Work))
                    .col(big_integer_null(TaskColumn::PlanAt))
                    .col(big_integer(TaskColumn::CreatedAt))
                    .col(big_integer(TaskColumn::FinishedAt).default(0))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TaskEntity).to_owned())
            .await
    }
}
