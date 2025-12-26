pub mod task;

use pm_backend::{AppState, Config};
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
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
            task::get_task_page_list
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
