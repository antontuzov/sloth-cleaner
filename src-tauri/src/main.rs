mod scanner;
mod cleaner;
mod ai;
mod database;
mod os;
mod utils;
mod types;
mod commands;

use types::AppState;
use tauri::Manager;

fn main() {
    env_logger::init();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::start_scan,
            commands::get_scan_progress,
            commands::get_scan_results,
            commands::propose_cleanup,
            commands::execute_cleanup,
            commands::list_snapshots,
            commands::restore_snapshot,
            commands::get_analytics,
            commands::ai_chat,
            commands::get_ai_recommendations,
            commands::update_user_feedback,
            commands::get_system_info,
            commands::export_logs,
            commands::reset_learning_data,
        ])
        .setup(|app| {
            let data_dir = app.path()
                .app_data_dir()
                .unwrap_or_else(|_| std::env::temp_dir().join("sloth-cleaner"));

            std::fs::create_dir_all(&data_dir).ok();

            let app_state = AppState::new(data_dir);

            database::init(app.handle())?;
            ai::init_model(app.handle())?;

            app.manage(app_state);

            log::info!("SlothCleaner initialized successfully");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
