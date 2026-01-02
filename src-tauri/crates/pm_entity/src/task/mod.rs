use crate::Error;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::{
    fmt::Debug,
    hash::{DefaultHasher, Hasher},
};

///工作的抽象
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Task {
    pub id: i32,
    //工作名称
    pub title: String,
    //工作状态
    pub completed: bool,
    //预期目标
    pub goal: Option<String>,
    //当前目标
    pub work: Option<String>,
    //计划时间
    pub plan_at: Option<i64>,
    //创建时间
    pub created_at: i64,
    //完成时间
    pub finished_at: Option<i64>,
    //时间花费
    pub duration: i64,
}

impl Task {
    pub fn get_hash(bytes: &[u8]) -> i64 {
        let mut hasher = DefaultHasher::new();
        hasher.write(bytes);

        i64::try_from(hasher.finish()).expect("task hash fail.")
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TaskUpdate {
    pub id: i32,
    pub title: String,
    pub completed: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TaskCreate {
    pub title: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TaskPageParams {
    pub page: u64,
    pub page_size: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PageResponse<T> {
    pub has_next: bool,
    pub page: u64,
    pub page_size: u64,
    pub data: Vec<T>,
    pub total: u64,
}

#[async_trait]
pub trait TaskRepo: Send + Sync + Debug {
    async fn create_task(&self, create: TaskCreate) -> Result<Task, Error>;

    async fn update_task(&self, update: TaskUpdate) -> Result<Task, Error>;

    async fn get_page_list(&self, params: TaskPageParams) -> Result<PageResponse<Task>, Error>;
}
