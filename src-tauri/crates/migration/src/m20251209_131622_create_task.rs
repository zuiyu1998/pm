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
                    .col(boolean(TaskColumn::Completed).default(false))
                    .col(boolean(TaskColumn::Delete).default(false))
                    .col(boolean(TaskColumn::Enable).default(true))
                    .col(string_null(TaskColumn::Goal))
                    .col(string_null(TaskColumn::Work))
                    .col(date_time_null(TaskColumn::PlanAt))
                    .col(date_time(TaskColumn::CreatedAt))
                    .col(big_integer(TaskColumn::Duration).default(0))
                    .col(date_time_null(TaskColumn::FinishedAt))
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
