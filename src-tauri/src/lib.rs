pub mod commands;
pub mod engine;
pub mod models;
pub mod narrative;
pub mod persistence;

use std::sync::Mutex;

use tauri::Manager;

use models::WorldState;
use persistence::database;
use persistence::save_load;
use persistence::state::{DbState, GameState, SettingsState};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data dir");
            std::fs::create_dir_all(&app_data_dir).ok();

            let db_path = app_data_dir.join("lorekeeper.db");
            let conn =
                rusqlite::Connection::open(&db_path).expect("Failed to open database");
            database::initialize_database(&conn).expect("Failed to initialize database");

            let settings = save_load::load_settings(&conn).unwrap_or_default();

            app.manage(DbState(Mutex::new(conn)));
            app.manage(GameState(Mutex::new(WorldState::default())));
            app.manage(SettingsState(Mutex::new(settings)));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::game::initialize_game,
            commands::game::new_game,
            commands::game::process_command,
            commands::save::save_game,
            commands::save::load_game,
            commands::save::list_saves,
            commands::save::delete_save,
            commands::settings::get_settings,
            commands::settings::update_settings,
            commands::settings::get_ollama_status,
            commands::settings::get_available_models,
            commands::autocomplete::get_completions,
            commands::map::get_map_data,
            commands::stats::get_stats,
            commands::stats::reset_stats,
            commands::combat::get_combat_log,
            commands::narration::rate_narration,
            commands::narration::retry_narration,
            commands::modules::list_modules,
            commands::modules::load_module,
            commands::themes::save_custom_theme,
            commands::themes::list_custom_themes,
            commands::themes::delete_custom_theme,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
