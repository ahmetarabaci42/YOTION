use crate::database::Database;
use crate::models::*;
use tauri::State;

#[tauri::command]
pub fn create_language(
    db: State<'_, Database>,
    req: CreateLanguageRequest,
) -> Result<Language, String> {
    db.create_language(req).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_languages(db: State<'_, Database>) -> Result<Vec<Language>, String> {
    db.get_languages().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn create_vocabulary(
    db: State<'_, Database>,
    req: CreateVocabularyRequest,
) -> Result<Vocabulary, String> {
    db.create_vocabulary(req).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_vocabulary_by_language(
    db: State<'_, Database>,
    language_id: i64,
) -> Result<Vec<Vocabulary>, String> {
    db.get_vocabulary_by_language(language_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_due_flashcards(
    db: State<'_, Database>,
    limit: Option<i64>,
) -> Result<Vec<(Flashcard, Vocabulary)>, String> {
    let limit = limit.unwrap_or(20);
    db.get_due_flashcards(limit).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn review_flashcard(
    db: State<'_, Database>,
    req: FlashcardReviewRequest,
) -> Result<(), String> {
    db.review_flashcard(req).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn create_tech_space(
    db: State<'_, Database>,
    req: CreateTechSpaceRequest,
) -> Result<TechSpace, String> {
    db.create_tech_space(req).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_tech_spaces(db: State<'_, Database>) -> Result<Vec<TechSpace>, String> {
    db.get_tech_spaces().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn create_code_snippet(
    db: State<'_, Database>,
    req: CreateCodeSnippetRequest,
) -> Result<CodeSnippet, String> {
    db.create_code_snippet(req).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_code_snippets_by_tech_space(
    db: State<'_, Database>,
    tech_space_id: i64,
) -> Result<Vec<CodeSnippet>, String> {
    db.get_code_snippets_by_tech_space(tech_space_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn search_code_snippets(
    db: State<'_, Database>,
    query: String,
    limit: Option<i64>,
) -> Result<Vec<CodeSnippet>, String> {
    let limit = limit.unwrap_or(50);
    db.search_code_snippets(&query, limit).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn create_project(
    db: State<'_, Database>,
    req: CreateProjectRequest,
) -> Result<Project, String> {
    db.create_project(req).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_projects(db: State<'_, Database>) -> Result<Vec<Project>, String> {
    db.get_projects().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn create_task(
    db: State<'_, Database>,
    req: CreateTaskRequest,
) -> Result<Task, String> {
    db.create_task(req).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_tasks_by_project(
    db: State<'_, Database>,
    project_id: i64,
) -> Result<Vec<Task>, String> {
    db.get_tasks_by_project(project_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_task_status(
    db: State<'_, Database>,
    task_id: i64,
    status: String,
) -> Result<(), String> {
    db.update_task_status(task_id, &status).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn create_event(
    db: State<'_, Database>,
    req: CreateEventRequest,
) -> Result<Event, String> {
    db.create_event(req).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_events_by_date(
    db: State<'_, Database>,
    date: String,
) -> Result<Vec<Event>, String> {
    db.get_events_by_date(&date).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn create_note(
    db: State<'_, Database>,
    req: CreateNoteRequest,
) -> Result<Note, String> {
    db.create_note(req).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_notes_by_date(
    db: State<'_, Database>,
    date: String,
) -> Result<Vec<Note>, String> {
    db.get_notes_by_date(&date).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub fn create_personal_account(
    db: State<'_, Database>,
    req: CreatePersonalAccountRequest,
) -> Result<PersonalAccount, String> {
    db.create_personal_account(req).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_personal_accounts(db: State<'_, Database>) -> Result<Vec<PersonalAccount>, String> {
    db.get_personal_accounts().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_personal_accounts_by_category(
    db: State<'_, Database>,
    category: String,
) -> Result<Vec<PersonalAccount>, String> {
    db.get_personal_accounts_by_category(&category).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn create_personal_info(
    db: State<'_, Database>,
    req: CreatePersonalInfoRequest,
) -> Result<PersonalInfo, String> {
    db.create_personal_info(req).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_personal_info(db: State<'_, Database>) -> Result<Vec<PersonalInfo>, String> {
    db.get_personal_info().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_personal_info_by_category(
    db: State<'_, Database>,
    category: String,
) -> Result<Vec<PersonalInfo>, String> {
    db.get_personal_info_by_category(&category).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn search_vocabulary(
    db: State<'_, Database>,
    query: String,
    limit: Option<i64>,
) -> Result<Vec<Vocabulary>, String> {
    let limit = limit.unwrap_or(50);
    db.search_vocabulary(&query, limit).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_language(
    db: State<'_, Database>,
    id: i64,
) -> Result<(), String> {
    db.delete_language(id).map_err(|e| e.to_string())
}
