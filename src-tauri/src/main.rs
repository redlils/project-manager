// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::commands::find_projects;

mod commands;
mod models;

fn main() {
  tauri::Builder::default()
      .setup(setup_app)
      .invoke_handler(tauri::generate_handler![find_projects])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

fn setup_app(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let appdata_path = app.handle().path_resolver().app_local_data_dir().unwrap();
    println!("Loading application data from {}", appdata_path.to_string_lossy());
    let connection = sqlite::open(appdata_path.join("config.db"))?;

    let init_query = "
        CREATE TABLE IF NOT EXISTS directories(location TEXT UNIQUE NOT NULL, single_project INTEGER NOT NULL);
        INSERT OR IGNORE INTO DIRECTORIES (location, single_project) VALUES ('D:\\School', 0), ('D:\\project-manager', 1), ('D:\\School\\SOFE2800', 1);
    ";

    connection.execute(init_query)?;

    Ok(())
}
