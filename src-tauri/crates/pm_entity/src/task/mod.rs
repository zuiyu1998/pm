use crate::Error;
use async_trait::async_trait;
use std::{
    fmt::Debug,
    hash::{DefaultHasher, Hasher},
};
use serde::{Deserialize, Serialize};

///工作的抽象
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Task {
    //工作名称
    pub title: String,
    //hash，名称的hash值，唯一id
    pub hash: u64,
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
    pub fn get_hash(bytes: &[u8]) -> u64 {
        let mut hasher = DefaultHasher::new();
        hasher.write(bytes);

        hasher.finish()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TaskCreate {
    pub title: String,
}

#[async_trait]
pub trait TaskRepo: Send + Sync + Debug {
    async fn create_task(&self, create: TaskCreate) -> Result<Task, Error>;
}
