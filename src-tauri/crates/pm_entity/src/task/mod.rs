use async_trait::async_trait;
use std::fmt::Debug;

pub struct Task {
    pub id: u64,
    pub title: String,
    pub completed: bool,
    pub created_at: u64,
    pub finished_at: Option<u64>,
}

pub enum Error {}

pub struct TaskCreate {
    pub title: String,
}

#[async_trait]
pub trait TaskRepo: Send + Sync + Debug {
    async fn create_task(&self, create: TaskCreate) -> Result<Task, Error>;
}
