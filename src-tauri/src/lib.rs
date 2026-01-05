pub mod task;
mod setup;

use pm_backend::{AppState, Config};
use tauri::Manager;
use tracing::info;

use crate::setup::setup_logging;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    setup_logging();

    info!("App start.");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                let config = Config::new();

                let value = AppState::init_app_state(&config)
                    .await
                    .expect("App state initialization failed.");
                app.manage(value);
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            task::create_task,
            task::get_task_page_list,
            task::update_task,
            task::delete_task,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
