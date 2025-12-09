use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "pm_task")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
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
    pub plan_at: Option<u64>,
    //创建时间
    pub created_at: u64,
    //完成时间
    pub finished_at: Option<u64>,
    //时间花费
    pub duration: u64,
}

impl ActiveModelBehavior for ActiveModel {}
