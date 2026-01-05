use pm_backend::AppState;
use pm_entity::{Error, TaskCreate, TaskPageParams, TaskUpdate};
use serde::Serialize;
use serde_json::{json, Value};
use tauri::State;

fn from_error(error: Error) -> Value {
    json!({
        "code": 502,
        "msg": error.to_string()
    })
}

fn from_payload<T: Serialize>(payload: T) -> Value {
    json!({
        "code": 200,
        "data": payload
    })
}

#[tauri::command]
pub async fn delete_task(id: i32, app_state: State<'_, AppState>) -> Result<Value, ()> {
    match app_state.db.task.delete_task(id).await {
        Ok(task) => Ok(from_payload(task)),
        Err(e) => Ok(from_error(e)),
    }
}

#[tauri::command]
pub async fn update_task(update: TaskUpdate, app_state: State<'_, AppState>) -> Result<Value, ()> {
    match app_state.db.task.update_task(update).await {
        Ok(task) => Ok(from_payload(task)),
        Err(e) => Ok(from_error(e)),
    }
}

#[tauri::command]
pub async fn create_task(create: TaskCreate, app_state: State<'_, AppState>) -> Result<Value, ()> {
    match app_state.db.task.create_task(create).await {
        Ok(task) => Ok(from_payload(task)),
        Err(e) => Ok(from_error(e)),
    }
}

#[tauri::command]
pub async fn get_task_page_list(
    params: TaskPageParams,
    app_state: State<'_, AppState>,
) -> Result<Value, ()> {
    match app_state.db.task.get_page_list(params).await {
        Ok(task) => Ok(from_payload(task)),
        Err(e) => Ok(from_error(e)),
    }
}
