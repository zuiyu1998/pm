use sea_orm::entity::prelude::*;
use chrono::NaiveDateTime;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "pm_task")]
pub struct Model {
    #[sea_orm(primary_key)]
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
    pub plan_at: Option<NaiveDateTime>,
    //创建时间
    pub created_at: NaiveDateTime,
    //完成时间
    pub finished_at: Option<NaiveDateTime>,
    //时间花费
    pub duration: i64,
    //是否删除
    pub delete: bool,
    //是否启用
    pub enable: bool,
}

impl ActiveModelBehavior for ActiveModel {}
