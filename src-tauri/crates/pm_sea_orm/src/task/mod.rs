mod model;

pub use model::{
    ActiveModel as TaskActiveModel, Column as TaskColumn, Entity as TaskEntity, Model as TaskModel,
};

use async_trait::async_trait;
use pm_entity::*;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityOrSelect,
    EntityTrait, IntoActiveModel, PaginatorTrait, QueryFilter,
};

use crate::utils::get_now_time;

#[derive(Debug)]
pub struct SeaOrmTaskRepo {
    conn: DatabaseConnection,
}

impl SeaOrmTaskRepo {
    pub fn new(conn: DatabaseConnection) -> Self {
        SeaOrmTaskRepo { conn }
    }
}

impl IntoActiveModel<TaskActiveModel> for TaskUpdate {
    fn into_active_model(self) -> TaskActiveModel {
        let mut active: TaskActiveModel = Default::default();

        let now = get_now_time();
        active.finished_at = Set(Some(now.timestamp_millis()));

        if let Some(completed) = self.completed {
            active.completed = Set(completed);
        }

        active.title = Set(self.title);
        active.id = Set(self.id);

        active
    }
}

impl IntoActiveModel<TaskActiveModel> for TaskCreate {
    fn into_active_model(self) -> TaskActiveModel {
        let mut active: TaskActiveModel = Default::default();

        let now = get_now_time();

        active.title = Set(self.title);
        active.created_at = Set(now.timestamp_millis());

        active
    }
}

impl From<TaskModel> for Task {
    fn from(value: TaskModel) -> Self {
        Task {
            id: value.id,
            title: value.title,
            completed: value.completed,
            goal: value.goal,
            work: value.work,
            plan_at: value.plan_at,
            created_at: value.created_at,
            finished_at: value.finished_at,
            duration: value.duration,
        }
    }
}

#[async_trait]
impl TaskRepo for SeaOrmTaskRepo {
    async fn delete_task(&self, id: i32) -> Result<(), Error> {
        let mut active_model: TaskActiveModel = Default::default();

        active_model.id = Set(id);
        active_model.delete = Set(false);
        active_model.enable = Set(false);

        active_model
            .update(&self.conn)
            .await
            .map_err(|e| Error::DbError(e.to_string()))?;

        Ok(())
    }

    async fn update_task(&self, update: TaskUpdate) -> Result<Task, Error> {
        let active_model = update.into_active_model();

        let model = active_model
            .update(&self.conn)
            .await
            .map_err(|e| Error::DbError(e.to_string()))?;

        Ok(model.into())
    }

    async fn create_task(&self, create: TaskCreate) -> Result<Task, Error> {
        let active_model = create.into_active_model();

        let model = active_model
            .insert(&self.conn)
            .await
            .map_err(|e| Error::DbError(e.to_string()))?;

        Ok(model.into())
    }

    async fn get_page_list(&self, params: TaskPageParams) -> Result<PageResponse<Task>, Error> {
        let total = TaskEntity::find()
            .count(&self.conn)
            .await
            .map_err(|e| Error::DbError(e.to_string()))?;

        let paginator = TaskEntity::find()
            .filter(TaskColumn::Enable.eq(true))
            .select()
            .paginate(&self.conn, params.page_size);

        let data = paginator
            .fetch_page(params.page)
            .await
            .map_err(|e| Error::DbError(e.to_string()))?
            .into_iter()
            .map(|model| model.into())
            .collect::<Vec<Task>>();

        let mut has_next = true;

        if (data.len() as u64) < params.page_size {
            has_next = false;
        }

        Ok(PageResponse {
            has_next,
            page: params.page,
            page_size: params.page_size,
            data,
            total,
        })
    }
}
