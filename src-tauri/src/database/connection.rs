use rusqlite::Connection;
use std::path::PathBuf;
use std::sync::Mutex;
use crate::errors::AppError;
use crate::models::*;
use crate::encryption::{encrypt_sensitive_data, decrypt_sensitive_data};

pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    pub fn new() -> Result<Self, AppError> {
        let db_path = Self::get_db_path()?;
        let conn = Connection::open(db_path)?;
        
        // Initialize database schema
        Self::init_schema(&conn)?;
        
        Ok(Database {
            conn: Mutex::new(conn),
        })
    }
    
    fn get_db_path() -> Result<PathBuf, AppError> {
        let mut path = dirs::data_dir().ok_or_else(|| AppError::Io("Could not find data directory".to_string()))?;
        path.push("yotion");
        std::fs::create_dir_all(&path)?;
        path.push("yotion.db");
        Ok(path)
    }
    
    fn init_schema(conn: &Connection) -> Result<(), AppError> {
        let schema = include_str!("../schema.sql");
        conn.execute_batch(schema)?;
        Ok(())
    }
}

// Language methods
impl Database {
    pub fn create_language(&self, req: CreateLanguageRequest) -> Result<Language, AppError> {
        let conn = self.conn.lock().unwrap();
        let now = chrono::Utc::now().to_rfc3339();
        
        conn.execute(
            "INSERT INTO languages (name, code, flag_emoji, created_at) VALUES (?1, ?2, ?3, ?4)",
            (req.name.clone(), req.code.clone(), req.flag_emoji.clone(), now.clone()),
        )?;
        
        let id = conn.last_insert_rowid();
        
        Ok(Language {
            id,
            name: req.name,
            code: req.code,
            flag_emoji: req.flag_emoji,
            created_at: now,
        })
    }
    
    pub fn get_languages(&self) -> Result<Vec<Language>, AppError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, name, code, flag_emoji, created_at FROM languages ORDER BY name")?;
        let rows = stmt.query_map([], |row| {
            Ok(Language {
                id: row.get(0)?,
                name: row.get(1)?,
                code: row.get(2)?,
                flag_emoji: row.get(3)?,
                created_at: row.get(4)?,
            })
        })?;
        
        let mut languages = Vec::new();
        for row in rows {
            languages.push(row?);
        }
        
        Ok(languages)
    }
    
    pub fn delete_language(&self, id: i64) -> Result<(), AppError> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM languages WHERE id = ?1", [id])?;
        Ok(())
    }
}

// Vocabulary methods
impl Database {
    pub fn create_vocabulary(&self, req: CreateVocabularyRequest) -> Result<Vocabulary, AppError> {
        let conn = self.conn.lock().unwrap();
        let now = chrono::Utc::now().to_rfc3339();
        
        conn.execute(
            "INSERT INTO vocabulary (language_id, word, translation, pronunciation, example_sentence, difficulty_level, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            (req.language_id, req.word.clone(), req.translation.clone(), req.pronunciation.clone(), req.example_sentence.clone(), req.difficulty_level, now.clone()),
        )?;
        
        let id = conn.last_insert_rowid();
        
        // Create flashcard for spaced repetition
        self.create_flashcard(id)?;
        
        Ok(Vocabulary {
            id,
            language_id: req.language_id,
            word: req.word,
            translation: req.translation,
            pronunciation: req.pronunciation,
            example_sentence: req.example_sentence,
            difficulty_level: req.difficulty_level,
            created_at: now,
        })
    }
    
    pub fn get_vocabulary_by_language(&self, language_id: i64) -> Result<Vec<Vocabulary>, AppError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, language_id, word, translation, pronunciation, example_sentence, difficulty_level, created_at 
             FROM vocabulary WHERE language_id = ?1 ORDER BY word"
        )?;
        
        let rows = stmt.query_map([language_id], |row| {
            Ok(Vocabulary {
                id: row.get(0)?,
                language_id: row.get(1)?,
                word: row.get(2)?,
                translation: row.get(3)?,
                pronunciation: row.get(4)?,
                example_sentence: row.get(5)?,
                difficulty_level: row.get(6)?,
                created_at: row.get(7)?,
            })
        })?;
        
        let mut vocabulary = Vec::new();
        for row in rows {
            vocabulary.push(row?);
        }
        
        Ok(vocabulary)
    }
    
    pub fn search_vocabulary(&self, query: &str, limit: i64) -> Result<Vec<Vocabulary>, AppError> {
        let conn = self.conn.lock().unwrap();
        let search_term = format!("%{}%", query);
        let mut stmt = conn.prepare(
            "SELECT id, language_id, word, translation, pronunciation, example_sentence, difficulty_level, created_at
             FROM vocabulary 
             WHERE word LIKE ?1 OR translation LIKE ?1 OR example_sentence LIKE ?1
             ORDER BY word
             LIMIT ?2"
        )?;
        
        let rows = stmt.query_map((search_term, limit), |row| {
            Ok(Vocabulary {
                id: row.get(0)?,
                language_id: row.get(1)?,
                word: row.get(2)?,
                translation: row.get(3)?,
                pronunciation: row.get(4)?,
                example_sentence: row.get(5)?,
                difficulty_level: row.get(6)?,
                created_at: row.get(7)?,
            })
        })?;
        
        let mut vocabulary = Vec::new();
        for row in rows {
            vocabulary.push(row?);
        }
        
        Ok(vocabulary)
    }
    
    pub fn get_due_flashcards(&self, limit: i64) -> Result<Vec<(Flashcard, Vocabulary)>, AppError> {
        let conn = self.conn.lock().unwrap();
        let now = chrono::Utc::now().to_rfc3339();
        
        let mut stmt = conn.prepare(
            "SELECT f.id, f.vocabulary_id, f.ease_factor, f.interval_days, f.repetitions, f.next_review, f.last_reviewed, f.created_at,
                    v.id, v.language_id, v.word, v.translation, v.pronunciation, v.example_sentence, v.difficulty_level, v.created_at
             FROM flashcards f
             JOIN vocabulary v ON f.vocabulary_id = v.id
             WHERE f.next_review <= ?1
             ORDER BY f.next_review
             LIMIT ?2"
        )?;
        
        let rows = stmt.query_map((now, limit), |row| {
            let flashcard = Flashcard {
                id: row.get(0)?,
                vocabulary_id: row.get(1)?,
                ease_factor: row.get(2)?,
                interval_days: row.get(3)?,
                repetitions: row.get(4)?,
                next_review: row.get(5)?,
                last_reviewed: row.get(6)?,
                created_at: row.get(7)?,
            };
            
            let vocabulary = Vocabulary {
                id: row.get(8)?,
                language_id: row.get(9)?,
                word: row.get(10)?,
                translation: row.get(11)?,
                pronunciation: row.get(12)?,
                example_sentence: row.get(13)?,
                difficulty_level: row.get(14)?,
                created_at: row.get(15)?,
            };
            
            Ok((flashcard, vocabulary))
        })?;
        
        let mut flashcards = Vec::new();
        for row in rows {
            flashcards.push(row?);
        }
        
        Ok(flashcards)
    }
    
    pub fn review_flashcard(&self, req: FlashcardReviewRequest) -> Result<(), AppError> {
        let conn = self.conn.lock().unwrap();
        
        // Get current flashcard data
        let mut stmt = conn.prepare(
            "SELECT ease_factor, interval_days, repetitions FROM flashcards WHERE id = ?1"
        )?;
        
        let (ease_factor, interval_days, repetitions): (f64, i64, i64) = stmt.query_row([req.flashcard_id], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?))
        })?;
        
        // Calculate new values based on quality (1-5)
        let quality = req.quality as f64;
        let new_ease_factor = if quality >= 3.0 {
            ease_factor + (0.1 - (5.0 - quality) * (0.08 + (5.0 - quality) * 0.02))
        } else {
            ease_factor.max(1.3)
        };
        
        let new_interval = if quality < 3.0 {
            1
        } else if repetitions == 0 {
            1
        } else if repetitions == 1 {
            6
        } else {
            (interval_days as f64 * new_ease_factor) as i64
        };
        
        let new_repetitions = if quality < 3.0 { 0 } else { repetitions + 1 };
        let next_review = chrono::Utc::now() + chrono::Duration::days(new_interval);
        let now = chrono::Utc::now().to_rfc3339();
        
        // Update flashcard
        conn.execute(
            "UPDATE flashcards SET ease_factor = ?1, interval_days = ?2, repetitions = ?3, next_review = ?4, last_reviewed = ?5 WHERE id = ?6",
            (new_ease_factor, new_interval, new_repetitions, next_review.to_rfc3339(), now, req.flashcard_id),
        )?;
        
        Ok(())
    }
    
    fn create_flashcard(&self, vocabulary_id: i64) -> Result<(), AppError> {
        let conn = self.conn.lock().unwrap();
        let now = chrono::Utc::now();
        let next_review = now + chrono::Duration::days(1);
        
        conn.execute(
            "INSERT INTO flashcards (vocabulary_id, ease_factor, interval_days, repetitions, next_review, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            (vocabulary_id, 2.5, 1, 0, next_review.to_rfc3339(), now.to_rfc3339()),
        )?;
        
        Ok(())
    }
}

// Tech Space methods
impl Database {
    pub fn create_tech_space(&self, req: CreateTechSpaceRequest) -> Result<TechSpace, AppError> {
        let conn = self.conn.lock().unwrap();
        let now = chrono::Utc::now().to_rfc3339();
        
        conn.execute(
            "INSERT INTO tech_spaces (name, description, icon, created_at) VALUES (?1, ?2, ?3, ?4)",
            (req.name.clone(), req.description.clone(), req.icon.clone(), now.clone()),
        )?;
        
        let id = conn.last_insert_rowid();
        
        Ok(TechSpace {
            id,
            name: req.name,
            description: req.description,
            icon: req.icon,
            created_at: now,
        })
    }
    
    pub fn get_tech_spaces(&self) -> Result<Vec<TechSpace>, AppError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, name, description, icon, created_at FROM tech_spaces ORDER BY name")?;
        let rows = stmt.query_map([], |row| {
            Ok(TechSpace {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                icon: row.get(3)?,
                created_at: row.get(4)?,
            })
        })?;
        
        let mut spaces = Vec::new();
        for row in rows {
            spaces.push(row?);
        }
        
        Ok(spaces)
    }
    
    pub fn create_code_snippet(&self, req: CreateCodeSnippetRequest) -> Result<CodeSnippet, AppError> {
        let conn = self.conn.lock().unwrap();
        let now = chrono::Utc::now().to_rfc3339();
        
        conn.execute(
            "INSERT INTO code_snippets (tech_space_id, title, description, code, language, tags, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            (req.tech_space_id, req.title.clone(), req.description.clone(), req.code.clone(), req.language.clone(), req.tags.clone(), now.clone(), now.clone()),
        )?;
        
        let id = conn.last_insert_rowid();
        
        Ok(CodeSnippet {
            id,
            tech_space_id: req.tech_space_id,
            title: req.title,
            description: req.description,
            code: req.code,
            language: req.language,
            tags: req.tags,
            created_at: now.clone(),
            updated_at: now,
        })
    }
    
    pub fn get_code_snippets_by_tech_space(&self, tech_space_id: i64) -> Result<Vec<CodeSnippet>, AppError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, tech_space_id, title, description, code, language, tags, created_at, updated_at 
             FROM code_snippets WHERE tech_space_id = ?1 ORDER BY created_at DESC"
        )?;
        
        let rows = stmt.query_map([tech_space_id], |row| {
            Ok(CodeSnippet {
                id: row.get(0)?,
                tech_space_id: row.get(1)?,
                title: row.get(2)?,
                description: row.get(3)?,
                code: row.get(4)?,
                language: row.get(5)?,
                tags: row.get(6)?,
                created_at: row.get(7)?,
                updated_at: row.get(8)?,
            })
        })?;
        
        let mut snippets = Vec::new();
        for row in rows {
            snippets.push(row?);
        }
        
        Ok(snippets)
    }
    
    pub fn search_code_snippets(&self, query: &str, limit: i64) -> Result<Vec<CodeSnippet>, AppError> {
        let conn = self.conn.lock().unwrap();
        let search_term = format!("%{}%", query);
        let mut stmt = conn.prepare(
            "SELECT id, tech_space_id, title, description, code, language, tags, created_at, updated_at
             FROM code_snippets 
             WHERE title LIKE ?1 OR description LIKE ?1 OR code LIKE ?1 OR tags LIKE ?1
             ORDER BY title
             LIMIT ?2"
        )?;
        
        let rows = stmt.query_map((search_term, limit), |row| {
            Ok(CodeSnippet {
                id: row.get(0)?,
                tech_space_id: row.get(1)?,
                title: row.get(2)?,
                description: row.get(3)?,
                code: row.get(4)?,
                language: row.get(5)?,
                tags: row.get(6)?,
                created_at: row.get(7)?,
                updated_at: row.get(8)?,
            })
        })?;
        
        let mut snippets = Vec::new();
        for row in rows {
            snippets.push(row?);
        }
        
        Ok(snippets)
    }
}

// Project methods
impl Database {
    pub fn create_project(&self, req: CreateProjectRequest) -> Result<Project, AppError> {
        let conn = self.conn.lock().unwrap();
        let now = chrono::Utc::now().to_rfc3339();
        
        conn.execute(
            "INSERT INTO projects (name, description, status, priority, start_date, end_date, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            (req.name.clone(), req.description.clone(), req.status.clone(), req.priority.clone(), req.start_date.clone(), req.end_date.clone(), now.clone(), now.clone()),
        )?;
        
        let id = conn.last_insert_rowid();
        
        Ok(Project {
            id,
            name: req.name,
            description: req.description,
            status: req.status,
            priority: req.priority,
            start_date: req.start_date,
            end_date: req.end_date,
            created_at: now.clone(),
            updated_at: now,
        })
    }
    
    pub fn get_projects(&self) -> Result<Vec<Project>, AppError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, name, description, status, priority, start_date, end_date, created_at, updated_at FROM projects ORDER BY created_at DESC")?;
        let rows = stmt.query_map([], |row| {
            Ok(Project {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                status: row.get(3)?,
                priority: row.get(4)?,
                start_date: row.get(5)?,
                end_date: row.get(6)?,
                created_at: row.get(7)?,
                updated_at: row.get(8)?,
            })
        })?;
        
        let mut projects = Vec::new();
        for row in rows {
            projects.push(row?);
        }
        
        Ok(projects)
    }
    
    pub fn create_task(&self, req: CreateTaskRequest) -> Result<Task, AppError> {
        let conn = self.conn.lock().unwrap();
        let now = chrono::Utc::now().to_rfc3339();
        
        conn.execute(
            "INSERT INTO tasks (project_id, title, description, status, priority, due_date, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            (req.project_id, req.title.clone(), req.description.clone(), req.status.clone(), req.priority.clone(), req.due_date.clone(), now.clone(), now.clone()),
        )?;
        
        let id = conn.last_insert_rowid();
        
        Ok(Task {
            id,
            project_id: req.project_id,
            title: req.title,
            description: req.description,
            status: req.status,
            priority: req.priority,
            due_date: req.due_date,
            completed_at: None,
            created_at: now.clone(),
            updated_at: now,
        })
    }
    
    pub fn get_tasks_by_project(&self, project_id: i64) -> Result<Vec<Task>, AppError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, project_id, title, description, status, priority, due_date, completed_at, created_at, updated_at 
             FROM tasks WHERE project_id = ?1 ORDER BY created_at DESC"
        )?;
        
        let rows = stmt.query_map([project_id], |row| {
            Ok(Task {
                id: row.get(0)?,
                project_id: row.get(1)?,
                title: row.get(2)?,
                description: row.get(3)?,
                status: row.get(4)?,
                priority: row.get(5)?,
                due_date: row.get(6)?,
                completed_at: row.get(7)?,
                created_at: row.get(8)?,
                updated_at: row.get(9)?,
            })
        })?;
        
        let mut tasks = Vec::new();
        for row in rows {
            tasks.push(row?);
        }
        
        Ok(tasks)
    }
    
    pub fn update_task_status(&self, task_id: i64, status: &str) -> Result<(), AppError> {
        let conn = self.conn.lock().unwrap();
        let now = chrono::Utc::now().to_rfc3339();
        let completed_at = if status == "completed" { Some(now.clone()) } else { None };
        
        conn.execute(
            "UPDATE tasks SET status = ?1, completed_at = ?2, updated_at = ?3 WHERE id = ?4",
            (status, completed_at, now, task_id),
        )?;
        
        Ok(())
    }
}

// Planner methods
impl Database {
    pub fn create_event(&self, req: CreateEventRequest) -> Result<Event, AppError> {
        let conn = self.conn.lock().unwrap();
        let now = chrono::Utc::now().to_rfc3339();
        
        conn.execute(
            "INSERT INTO events (title, description, event_date, start_time, end_time, event_type, priority, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            (req.title.clone(), req.description.clone(), req.event_date.clone(), req.start_time.clone(), req.end_time.clone(), req.event_type.clone(), req.priority.clone(), now.clone(), now.clone()),
        )?;
        
        let id = conn.last_insert_rowid();
        
        Ok(Event {
            id,
            title: req.title,
            description: req.description,
            event_date: req.event_date,
            start_time: req.start_time,
            end_time: req.end_time,
            event_type: req.event_type,
            priority: req.priority,
            created_at: now.clone(),
            updated_at: now,
        })
    }
    
    pub fn get_events_by_date(&self, date: &str) -> Result<Vec<Event>, AppError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, title, description, event_date, start_time, end_time, event_type, priority, created_at, updated_at 
             FROM events WHERE event_date = ?1 ORDER BY start_time"
        )?;
        
        let rows = stmt.query_map([date], |row| {
            Ok(Event {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                event_date: row.get(3)?,
                start_time: row.get(4)?,
                end_time: row.get(5)?,
                event_type: row.get(6)?,
                priority: row.get(7)?,
                created_at: row.get(8)?,
                updated_at: row.get(9)?,
            })
        })?;
        
        let mut events = Vec::new();
        for row in rows {
            events.push(row?);
        }
        
        Ok(events)
    }
    
    pub fn create_note(&self, req: CreateNoteRequest) -> Result<Note, AppError> {
        let conn = self.conn.lock().unwrap();
        let now = chrono::Utc::now().to_rfc3339();
        
        conn.execute(
            "INSERT INTO notes (title, content, note_date, tags, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            (req.title.clone(), req.content.clone(), req.note_date.clone(), req.tags.clone(), now.clone(), now.clone()),
        )?;
        
        let id = conn.last_insert_rowid();
        
        Ok(Note {
            id,
            title: req.title,
            content: req.content,
            note_date: req.note_date,
            tags: req.tags,
            created_at: now.clone(),
            updated_at: now,
        })
    }
    
    pub fn get_notes_by_date(&self, date: &str) -> Result<Vec<Note>, AppError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, title, content, note_date, tags, created_at, updated_at 
             FROM notes WHERE note_date = ?1 ORDER BY created_at DESC"
        )?;
        
        let rows = stmt.query_map([date], |row| {
            Ok(Note {
                id: row.get(0)?,
                title: row.get(1)?,
                content: row.get(2)?,
                note_date: row.get(3)?,
                tags: row.get(4)?,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
            })
        })?;
        
        let mut notes = Vec::new();
        for row in rows {
            notes.push(row?);
        }
        
        Ok(notes)
    }
}

// Personal Vault methods
impl Database {
    pub fn create_personal_account(&self, req: CreatePersonalAccountRequest) -> Result<PersonalAccount, AppError> {
        let conn = self.conn.lock().unwrap();
        let now = chrono::Utc::now().to_rfc3339();
        
        // Encrypt sensitive data
        let encrypted_password = encrypt_sensitive_data(&req.password)?;
        let encrypted_email = encrypt_sensitive_data(&req.email)?;
        
        conn.execute(
            "INSERT INTO personal_accounts (title, email, password, website, notes, category, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            (req.title.clone(), encrypted_email, encrypted_password, req.website.clone(), req.notes.clone(), req.category.clone(), now.clone(), now.clone()),
        )?;
        
        let id = conn.last_insert_rowid();
        
        Ok(PersonalAccount {
            id,
            title: req.title,
            email: req.email, // Return unencrypted for display
            password: req.password, // Return unencrypted for display
            website: req.website,
            notes: req.notes,
            category: req.category,
            created_at: now.clone(),
            updated_at: now,
        })
    }
    
    pub fn get_personal_accounts(&self) -> Result<Vec<PersonalAccount>, AppError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, title, email, password, website, notes, category, created_at, updated_at FROM personal_accounts ORDER BY created_at DESC")?;
        let rows = stmt.query_map([], |row| {
            let encrypted_email: String = row.get(2)?;
            let encrypted_password: String = row.get(3)?;
            
            // Decrypt sensitive data
            let email = decrypt_sensitive_data(&encrypted_email).unwrap_or_else(|_| "Decryption failed".to_string());
            let password = decrypt_sensitive_data(&encrypted_password).unwrap_or_else(|_| "Decryption failed".to_string());
            
            Ok(PersonalAccount {
                id: row.get(0)?,
                title: row.get(1)?,
                email,
                password,
                website: row.get(4)?,
                notes: row.get(5)?,
                category: row.get(6)?,
                created_at: row.get(7)?,
                updated_at: row.get(8)?,
            })
        })?;
        
        let mut accounts = Vec::new();
        for row in rows {
            accounts.push(row?);
        }
        
        Ok(accounts)
    }
    
    pub fn get_personal_accounts_by_category(&self, category: &str) -> Result<Vec<PersonalAccount>, AppError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, title, email, password, website, notes, category, created_at, updated_at 
             FROM personal_accounts WHERE category = ?1 ORDER BY created_at DESC"
        )?;
        
        let rows = stmt.query_map([category], |row| {
            let encrypted_email: String = row.get(2)?;
            let encrypted_password: String = row.get(3)?;
            
            // Decrypt sensitive data
            let email = decrypt_sensitive_data(&encrypted_email).unwrap_or_else(|_| "Decryption failed".to_string());
            let password = decrypt_sensitive_data(&encrypted_password).unwrap_or_else(|_| "Decryption failed".to_string());
            
            Ok(PersonalAccount {
                id: row.get(0)?,
                title: row.get(1)?,
                email,
                password,
                website: row.get(4)?,
                notes: row.get(5)?,
                category: row.get(6)?,
                created_at: row.get(7)?,
                updated_at: row.get(8)?,
            })
        })?;
        
        let mut accounts = Vec::new();
        for row in rows {
            accounts.push(row?);
        }
        
        Ok(accounts)
    }
    
    pub fn create_personal_info(&self, req: CreatePersonalInfoRequest) -> Result<PersonalInfo, AppError> {
        let conn = self.conn.lock().unwrap();
        let now = chrono::Utc::now().to_rfc3339();
        
        // Encrypt sensitive data if marked as sensitive
        let content = if req.is_sensitive {
            encrypt_sensitive_data(&req.content)?
        } else {
            req.content.clone()
        };
        
        conn.execute(
            "INSERT INTO personal_info (title, content, category, is_sensitive, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            (req.title.clone(), content, req.category.clone(), req.is_sensitive, now.clone(), now.clone()),
        )?;
        
        let id = conn.last_insert_rowid();
        
        Ok(PersonalInfo {
            id,
            title: req.title,
            content: req.content, // Return unencrypted for display
            category: req.category,
            is_sensitive: req.is_sensitive,
            created_at: now.clone(),
            updated_at: now,
        })
    }
    
    pub fn get_personal_info(&self) -> Result<Vec<PersonalInfo>, AppError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, title, content, category, is_sensitive, created_at, updated_at FROM personal_info ORDER BY created_at DESC")?;
        let rows = stmt.query_map([], |row| {
            let encrypted_content: String = row.get(2)?;
            let is_sensitive: bool = row.get(4)?;
            
            // Decrypt sensitive data if needed
            let content = if is_sensitive {
                decrypt_sensitive_data(&encrypted_content).unwrap_or_else(|_| "Decryption failed".to_string())
            } else {
                encrypted_content
            };
            
            Ok(PersonalInfo {
                id: row.get(0)?,
                title: row.get(1)?,
                content,
                category: row.get(3)?,
                is_sensitive,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
            })
        })?;
        
        let mut info = Vec::new();
        for row in rows {
            info.push(row?);
        }
        
        Ok(info)
    }
    
    pub fn get_personal_info_by_category(&self, category: &str) -> Result<Vec<PersonalInfo>, AppError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, title, content, category, is_sensitive, created_at, updated_at 
             FROM personal_info WHERE category = ?1 ORDER BY created_at DESC"
        )?;
        
        let rows = stmt.query_map([category], |row| {
            let encrypted_content: String = row.get(2)?;
            let is_sensitive: bool = row.get(4)?;
            
            // Decrypt sensitive data if needed
            let content = if is_sensitive {
                decrypt_sensitive_data(&encrypted_content).unwrap_or_else(|_| "Decryption failed".to_string())
            } else {
                encrypted_content
            };
            
            Ok(PersonalInfo {
                id: row.get(0)?,
                title: row.get(1)?,
                content,
                category: row.get(3)?,
                is_sensitive,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
            })
        })?;
        
        let mut info = Vec::new();
        for row in rows {
            info.push(row?);
        }
        
        Ok(info)
    }
}