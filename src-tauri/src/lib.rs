pub mod database;
mod commands;
mod models;
mod errors;
mod validation;
mod encryption;

use database::Database;
use commands::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(Database::new().expect("Failed to initialize database"))
        .invoke_handler(tauri::generate_handler![
            greet,
            create_language,
            get_languages,
            delete_language,
            create_vocabulary,
            get_vocabulary_by_language,
            search_vocabulary,
            get_due_flashcards,
            review_flashcard,
            create_tech_space,
            get_tech_spaces,
            create_code_snippet,
            get_code_snippets_by_tech_space,
            search_code_snippets,
            create_project,
            get_projects,
            create_task,
            get_tasks_by_project,
            update_task_status,
            create_event,
            get_events_by_date,
            create_note,
            get_notes_by_date,
            create_personal_account,
            get_personal_accounts,
            get_personal_accounts_by_category,
            create_personal_info,
            get_personal_info,
            get_personal_info_by_category
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
