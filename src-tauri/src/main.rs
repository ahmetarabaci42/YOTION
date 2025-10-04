// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod models;
mod db;
mod commands;
mod validation;
mod encryption;
mod errors;

use db::Database;

fn main() {
    let db = Database::new().expect("Failed to initialize database");
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(db)
        .invoke_handler(tauri::generate_handler![
            commands::greet,
            commands::create_language,
            commands::get_languages,
            commands::create_vocabulary,
            commands::get_vocabulary_by_language,
            commands::get_due_flashcards,
            commands::review_flashcard,
            commands::create_tech_space,
            commands::get_tech_spaces,
            commands::create_code_snippet,
            commands::get_code_snippets_by_tech_space,
            commands::search_code_snippets,
            commands::create_project,
            commands::get_projects,
            commands::create_task,
            commands::get_tasks_by_project,
            commands::update_task_status,
            commands::create_event,
            commands::get_events_by_date,
            commands::create_note,
            commands::get_notes_by_date,
            commands::create_personal_account,
            commands::get_personal_accounts,
            commands::get_personal_accounts_by_category,
            commands::create_personal_info,
            commands::get_personal_info,
            commands::get_personal_info_by_category,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
