use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Language {
    pub id: i64,
    pub name: String,
    pub code: String,
    pub flag_emoji: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Vocabulary {
    pub id: i64,
    pub language_id: i64,
    pub word: String,
    pub translation: String,
    pub pronunciation: Option<String>,
    pub example_sentence: Option<String>,
    pub difficulty_level: i32,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Flashcard {
    pub id: i64,
    pub vocabulary_id: i64,
    pub ease_factor: f64,
    pub interval_days: i32,
    pub repetitions: i32,
    pub next_review: String,
    pub last_reviewed: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TechSpace {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub icon: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CodeSnippet {
    pub id: i64,
    pub tech_space_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub code: String,
    pub language: String,
    pub tags: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

// Request/Response DTOs
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateLanguageRequest {
    pub name: String,
    pub code: String,
    pub flag_emoji: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateVocabularyRequest {
    pub language_id: i64,
    pub word: String,
    pub translation: String,
    pub pronunciation: Option<String>,
    pub example_sentence: Option<String>,
    pub difficulty_level: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTechSpaceRequest {
    pub name: String,
    pub description: Option<String>,
    pub icon: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCodeSnippetRequest {
    pub tech_space_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub code: String,
    pub language: String,
    pub tags: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub status: String,
    pub priority: String,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: i64,
    pub project_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub priority: String,
    pub due_date: Option<String>,
    pub completed_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,
    pub description: Option<String>,
    pub status: String,
    pub priority: String,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTaskRequest {
    pub project_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub priority: String,
    pub due_date: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Event {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub event_date: String,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub event_type: String,
    pub priority: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Note {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub note_date: String,
    pub tags: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateEventRequest {
    pub title: String,
    pub description: Option<String>,
    pub event_date: String,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub event_type: String,
    pub priority: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateNoteRequest {
    pub title: String,
    pub content: String,
    pub note_date: String,
    pub tags: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FlashcardReviewRequest {
    pub flashcard_id: i64,
    pub quality: i32, // 0-5 rating
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PersonalAccount {
    pub id: i64,
    pub title: String,
    pub email: String,
    pub password: String,
    pub website: Option<String>,
    pub notes: Option<String>,
    pub category: String, // email, social, banking, work, etc.
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PersonalInfo {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub category: String, // identity, documents, contacts, etc.
    pub is_sensitive: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePersonalAccountRequest {
    pub title: String,
    pub email: String,
    pub password: String,
    pub website: Option<String>,
    pub notes: Option<String>,
    pub category: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePersonalInfoRequest {
    pub title: String,
    pub content: String,
    pub category: String,
    pub is_sensitive: bool,
}
