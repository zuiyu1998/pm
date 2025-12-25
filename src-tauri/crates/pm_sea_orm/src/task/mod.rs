mod model;

pub use model::{
    ActiveModel as TaskActiveModel, Column as TaskColumn, Entity as TaskEntity, Model as TaskModel,
};

use async_trait::async_trait;
use pm_entity::*;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, IntoActiveModel};

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

impl IntoActiveModel<TaskActiveModel> for TaskCreate {
    fn into_active_model(self) -> TaskActiveModel {
        let mut active: TaskActiveModel = Default::default();

        let now = get_now_time();

        active.hash = Set(Task::get_hash(self.title.as_bytes()));
        active.title = Set(self.title);
        active.created_at = Set(now.timestamp_millis());

        active
    }
}

impl From<TaskModel> for Task {
    fn from(value: TaskModel) -> Self {
        Task {
            title: value.title,
            hash: value.hash,
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
    async fn create_task(&self, create: TaskCreate) -> Result<Task, Error> {
        let active_model = create.into_active_model();

        let model = active_model
            .insert(&self.conn)
            .await
            .map_err(|e| Error::DbError(e.to_string()))?;

        Ok(model.into())
    }
}
