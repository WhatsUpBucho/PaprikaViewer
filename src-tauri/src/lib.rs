pub mod commands;
pub mod db;
pub mod error;
pub mod keychain;
pub mod photos;

use std::path::PathBuf;
use std::sync::Arc;
use tauri::Manager;
use tokio::sync::Mutex;

pub struct AppState {
    pub db: tokio_rusqlite::Connection,
    pub token: Arc<Mutex<Option<String>>>,
    pub data_dir: PathBuf,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let data_dir = app
                .path()
                .app_data_dir()
                .expect("failed to resolve app data dir");
            std::fs::create_dir_all(&data_dir)?;

            let photos_dir = data_dir.join("photos");
            std::fs::create_dir_all(&photos_dir)?;

            let db_path = data_dir.join("recipes.db");

            let db = tauri::async_runtime::block_on(async {
                tokio_rusqlite::Connection::open(&db_path)
                    .await
                    .expect("failed to open database")
            });

            tauri::async_runtime::block_on(async {
                db.call(|conn| {
                    db::schema::run_migrations(conn)
                        .map_err(|e| tokio_rusqlite::Error::Other(Box::new(e)))
                })
                .await
                .expect("database migrations failed")
            });

            let stored_token = keychain::load_token().ok();

            app.manage(AppState {
                db,
                token: Arc::new(Mutex::new(stored_token)),
                data_dir,
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::auth::login,
            commands::auth::logout,
            commands::auth::check_auth,
            commands::sync::sync_recipes,
            commands::recipes::get_recipes,
            commands::recipes::get_recipe_detail,
            commands::recipes::get_categories,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application")
}
