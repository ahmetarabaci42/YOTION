use rusqlite::{Connection, Result as SqliteResult};
use anyhow::Result;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use crate::models::*;
use chrono::Utc;

pub struct Database {
    conn: Arc<Mutex<Connection>>,
}

impl Database {
    pub fn new() -> Result<Self> {
        let app_data_dir = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("yotion");
        
        std::fs::create_dir_all(&app_data_dir)?;
        let db_path = app_data_dir.join("yotion.db");
        
        let conn = Connection::open(db_path)?;
        let db = Database { 
            conn: Arc::new(Mutex::new(conn))
        };
        
        db.create_tables()?;
        Ok(db)
    }

    fn create_tables(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS languages (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE,
                code TEXT NOT NULL UNIQUE,
                flag_emoji TEXT NOT NULL,
                created_at TEXT NOT NULL
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS vocabulary (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                language_id INTEGER NOT NULL,
                word TEXT NOT NULL,
                translation TEXT NOT NULL,
                pronunciation TEXT,
                example_sentence TEXT,
                difficulty_level INTEGER NOT NULL DEFAULT 1,
                created_at TEXT NOT NULL,
                FOREIGN KEY (language_id) REFERENCES languages(id) ON DELETE CASCADE
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS flashcards (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                vocabulary_id INTEGER NOT NULL,
                ease_factor REAL NOT NULL DEFAULT 2.5,
                interval_days INTEGER NOT NULL DEFAULT 1,
                repetitions INTEGER NOT NULL DEFAULT 0,
                next_review TEXT NOT NULL,
                last_reviewed TEXT,
                created_at TEXT NOT NULL,
                FOREIGN KEY (vocabulary_id) REFERENCES vocabulary(id) ON DELETE CASCADE
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS tech_spaces (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE,
                description TEXT,
                icon TEXT NOT NULL,
                created_at TEXT NOT NULL
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS code_snippets (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                tech_space_id INTEGER NOT NULL,
                title TEXT NOT NULL,
                description TEXT,
                code TEXT NOT NULL,
                language TEXT NOT NULL,
                tags TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                FOREIGN KEY (tech_space_id) REFERENCES tech_spaces(id) ON DELETE CASCADE
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS projects (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                description TEXT,
                status TEXT NOT NULL DEFAULT 'active',
                priority TEXT NOT NULL DEFAULT 'medium',
                start_date TEXT,
                end_date TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS tasks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                project_id INTEGER NOT NULL,
                title TEXT NOT NULL,
                description TEXT,
                status TEXT NOT NULL DEFAULT 'todo',
                priority TEXT NOT NULL DEFAULT 'medium',
                due_date TEXT,
                completed_at TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS events (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                description TEXT,
                event_date TEXT NOT NULL,
                start_time TEXT,
                end_time TEXT,
                event_type TEXT NOT NULL DEFAULT 'event',
                priority TEXT NOT NULL DEFAULT 'medium',
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS notes (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                content TEXT NOT NULL,
                note_date TEXT NOT NULL,
                tags TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        )?;

        Ok(())
    }

    pub fn create_language(&self, req: CreateLanguageRequest) -> Result<Language> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();
        
        conn.execute(
            "INSERT INTO languages (name, code, flag_emoji, created_at) VALUES (?1, ?2, ?3, ?4)",
            (&req.name, &req.code, &req.flag_emoji, &now),
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

    pub fn get_languages(&self) -> Result<Vec<Language>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, name, code, flag_emoji, created_at FROM languages ORDER BY created_at DESC")?;
        
        let language_iter = stmt.query_map([], |row| {
            Ok(Language {
                id: row.get(0)?,
                name: row.get(1)?,
                code: row.get(2)?,
                flag_emoji: row.get(3)?,
                created_at: row.get(4)?,
            })
        })?;

        let mut languages = Vec::new();
        for language in language_iter {
            languages.push(language?);
        }
        
        Ok(languages)
    }

    pub fn create_vocabulary(&self, req: CreateVocabularyRequest) -> Result<Vocabulary> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();
        
        conn.execute(
            "INSERT INTO vocabulary (language_id, word, translation, pronunciation, example_sentence, difficulty_level, created_at) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            (&req.language_id, &req.word, &req.translation, &req.pronunciation, &req.example_sentence, &req.difficulty_level, &now),
        )?;
        
        let vocab_id = conn.last_insert_rowid();
        
        // Create flashcard for this vocabulary
        conn.execute(
            "INSERT INTO flashcards (vocabulary_id, ease_factor, interval_days, repetitions, next_review, created_at)
             VALUES (?1, 2.5, 1, 0, ?2, ?3)",
            (vocab_id, &now, &now),
        )?;

        Ok(Vocabulary {
            id: vocab_id,
            language_id: req.language_id,
            word: req.word,
            translation: req.translation,
            pronunciation: req.pronunciation,
            example_sentence: req.example_sentence,
            difficulty_level: req.difficulty_level,
            created_at: now,
        })
    }

    pub fn get_vocabulary_by_language(&self, language_id: i64) -> Result<Vec<Vocabulary>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, language_id, word, translation, pronunciation, example_sentence, difficulty_level, created_at 
             FROM vocabulary WHERE language_id = ?1 ORDER BY created_at DESC"
        )?;
        
        let vocab_iter = stmt.query_map([language_id], |row| {
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
        for vocab in vocab_iter {
            vocabulary.push(vocab?);
        }
        
        Ok(vocabulary)
    }

    pub fn get_due_flashcards(&self, limit: i64) -> Result<Vec<(Flashcard, Vocabulary)>> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();
        
        let mut stmt = conn.prepare(
            "SELECT f.id, f.vocabulary_id, f.ease_factor, f.interval_days, f.repetitions, 
                    f.next_review, f.last_reviewed, f.created_at,
                    v.language_id, v.word, v.translation, v.pronunciation, v.example_sentence, v.difficulty_level, v.created_at as vocab_created_at
             FROM flashcards f
             JOIN vocabulary v ON f.vocabulary_id = v.id
             WHERE f.next_review <= ?1
             ORDER BY f.next_review ASC
             LIMIT ?2"
        )?;

        let flashcard_iter = stmt.query_map([&now, &limit.to_string()], |row| {
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
                id: row.get(1)?,
                language_id: row.get(8)?,
                word: row.get(9)?,
                translation: row.get(10)?,
                pronunciation: row.get(11)?,
                example_sentence: row.get(12)?,
                difficulty_level: row.get(13)?,
                created_at: row.get(14)?,
            };
            
            Ok((flashcard, vocabulary))
        })?;

        let mut result = Vec::new();
        for item in flashcard_iter {
            result.push(item?);
        }
        
        Ok(result)
    }

    pub fn review_flashcard(&self, req: FlashcardReviewRequest) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        
        // Get current flashcard data
        let mut stmt = conn.prepare("SELECT ease_factor, interval_days, repetitions FROM flashcards WHERE id = ?1")?;
        let (mut ease_factor, mut interval, mut repetitions): (f64, i32, i32) = stmt.query_row([req.flashcard_id], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?))
        })?;

        // SM-2 algorithm implementation
        let quality = req.quality.max(0).min(5);

        if quality >= 3 {
            if repetitions == 0 {
                interval = 1;
            } else if repetitions == 1 {
                interval = 6;
            } else {
                interval = (interval as f64 * ease_factor).round() as i32;
            }
            repetitions += 1;
        } else {
            repetitions = 0;
            interval = 1;
        }

        ease_factor = ease_factor + (0.1 - (5.0 - quality as f64) * (0.08 + (5.0 - quality as f64) * 0.02));
        ease_factor = ease_factor.max(1.3);

        let next_review = (Utc::now() + chrono::Duration::days(interval as i64)).to_rfc3339();
        let now = Utc::now().to_rfc3339();

        conn.execute(
            "UPDATE flashcards SET ease_factor = ?1, interval_days = ?2, repetitions = ?3, 
             next_review = ?4, last_reviewed = ?5 WHERE id = ?6",
            (ease_factor, interval, repetitions, &next_review, &now, req.flashcard_id),
        )?;

        Ok(())
    }

    // Tech Space operations
    pub fn create_tech_space(&self, req: CreateTechSpaceRequest) -> Result<TechSpace> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();
        
        conn.execute(
            "INSERT INTO tech_spaces (name, description, icon, created_at) VALUES (?1, ?2, ?3, ?4)",
            (&req.name, &req.description, &req.icon, &now),
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

    pub fn get_tech_spaces(&self) -> Result<Vec<TechSpace>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, name, description, icon, created_at FROM tech_spaces ORDER BY created_at DESC")?;
        
        let tech_space_iter = stmt.query_map([], |row| {
            Ok(TechSpace {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                icon: row.get(3)?,
                created_at: row.get(4)?,
            })
        })?;

        let mut tech_spaces = Vec::new();
        for tech_space in tech_space_iter {
            tech_spaces.push(tech_space?);
        }
        
        Ok(tech_spaces)
    }

    // Code Snippet operations
    pub fn create_code_snippet(&self, req: CreateCodeSnippetRequest) -> Result<CodeSnippet> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();
        
        conn.execute(
            "INSERT INTO code_snippets (tech_space_id, title, description, code, language, tags, created_at, updated_at) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            (&req.tech_space_id, &req.title, &req.description, &req.code, &req.language, &req.tags, &now, &now),
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

    pub fn get_code_snippets_by_tech_space(&self, tech_space_id: i64) -> Result<Vec<CodeSnippet>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, tech_space_id, title, description, code, language, tags, created_at, updated_at 
             FROM code_snippets WHERE tech_space_id = ?1 ORDER BY updated_at DESC"
        )?;
        
        let snippet_iter = stmt.query_map([tech_space_id], |row| {
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
        for snippet in snippet_iter {
            snippets.push(snippet?);
        }
        
        Ok(snippets)
    }

    pub fn search_code_snippets(&self, query: &str, limit: i64) -> Result<Vec<CodeSnippet>> {
        let conn = self.conn.lock().unwrap();
        let search_query = format!("%{}%", query);
        
        let mut stmt = conn.prepare(
            "SELECT id, tech_space_id, title, description, code, language, tags, created_at, updated_at 
             FROM code_snippets WHERE title LIKE ?1 OR description LIKE ?1 OR code LIKE ?1 OR tags LIKE ?1
             ORDER BY updated_at DESC LIMIT ?2"
        )?;
        
        let snippet_iter = stmt.query_map([&search_query, &limit.to_string()], |row| {
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
        for snippet in snippet_iter {
            snippets.push(snippet?);
        }
        
        Ok(snippets)
    }

    // Project operations
    pub fn create_project(&self, req: CreateProjectRequest) -> Result<Project> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();
        
        conn.execute(
            "INSERT INTO projects (name, description, status, priority, start_date, end_date, created_at, updated_at) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            (&req.name, &req.description, &req.status, &req.priority, &req.start_date, &req.end_date, &now, &now),
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

    pub fn get_projects(&self) -> Result<Vec<Project>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, name, description, status, priority, start_date, end_date, created_at, updated_at FROM projects ORDER BY updated_at DESC")?;
        
        let project_iter = stmt.query_map([], |row| {
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
        for project in project_iter {
            projects.push(project?);
        }
        
        Ok(projects)
    }

    // Task operations
    pub fn create_task(&self, req: CreateTaskRequest) -> Result<Task> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();
        
        conn.execute(
            "INSERT INTO tasks (project_id, title, description, status, priority, due_date, created_at, updated_at) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            (&req.project_id, &req.title, &req.description, &req.status, &req.priority, &req.due_date, &now, &now),
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

    pub fn get_tasks_by_project(&self, project_id: i64) -> Result<Vec<Task>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, project_id, title, description, status, priority, due_date, completed_at, created_at, updated_at 
             FROM tasks WHERE project_id = ?1 ORDER BY created_at ASC"
        )?;
        
        let task_iter = stmt.query_map([project_id], |row| {
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
        for task in task_iter {
            tasks.push(task?);
        }
        
        Ok(tasks)
    }

    pub fn update_task_status(&self, task_id: i64, status: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();
        let completed_at = if status == "completed" { Some(now.clone()) } else { None };
        
        conn.execute(
            "UPDATE tasks SET status = ?1, completed_at = ?2, updated_at = ?3 WHERE id = ?4",
            (status, &completed_at, &now, task_id),
        )?;

        Ok(())
    }

    // Event operations
    pub fn create_event(&self, req: CreateEventRequest) -> Result<Event> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();
        
        conn.execute(
            "INSERT INTO events (title, description, event_date, start_time, end_time, event_type, priority, created_at, updated_at) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            (&req.title, &req.description, &req.event_date, &req.start_time, &req.end_time, &req.event_type, &req.priority, &now, &now),
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

    pub fn get_events_by_date(&self, date: &str) -> Result<Vec<Event>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, title, description, event_date, start_time, end_time, event_type, priority, created_at, updated_at 
             FROM events WHERE event_date = ?1 ORDER BY start_time ASC"
        )?;
        
        let event_iter = stmt.query_map([date], |row| {
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
        for event in event_iter {
            events.push(event?);
        }
        
        Ok(events)
    }

    // Note operations
    pub fn create_note(&self, req: CreateNoteRequest) -> Result<Note> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();
        
        conn.execute(
            "INSERT INTO notes (title, content, note_date, tags, created_at, updated_at) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            (&req.title, &req.content, &req.note_date, &req.tags, &now, &now),
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

    pub fn get_notes_by_date(&self, date: &str) -> Result<Vec<Note>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, title, content, note_date, tags, created_at, updated_at 
             FROM notes WHERE note_date = ?1 ORDER BY created_at DESC"
        )?;
        
        let note_iter = stmt.query_map([date], |row| {
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
        for note in note_iter {
            notes.push(note?);
        }
        
        Ok(notes)
    }
}
