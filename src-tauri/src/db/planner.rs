use crate::models::*;
use crate::db::Database;
use crate::validation::Validator;
use anyhow::Result;
use chrono::Utc;

impl Database {
    pub fn create_event(&self, req: CreateEventRequest) -> Result<Event> {
        Validator::validate_not_empty(&req.title, "Event title")?;
        Validator::validate_string_length(&req.title, "Event title", 1, 200)?;
        Validator::validate_date_format(&req.event_date)?;
        Validator::validate_priority(&req.priority)?;

        let title = Validator::sanitize_string(req.title);
        let description = Validator::sanitize_optional_string(req.description);
        let event_type = Validator::sanitize_string(req.event_type);
        let priority = Validator::sanitize_string(req.priority);

        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();

        conn.execute(
            "INSERT INTO events (title, description, event_date, start_time, end_time, event_type, priority, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            (&title, &description, &req.event_date, &req.start_time, &req.end_time, &event_type, &priority, &now, &now),
        )?;

        let id = conn.last_insert_rowid();

        Ok(Event {
            id,
            title,
            description,
            event_date: req.event_date,
            start_time: req.start_time,
            end_time: req.end_time,
            event_type,
            priority,
            created_at: now.clone(),
            updated_at: now,
        })
    }

    pub fn get_events_by_date(&self, date: &str) -> Result<Vec<Event>> {
        Validator::validate_date_format(date)?;

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

    pub fn create_note(&self, req: CreateNoteRequest) -> Result<Note> {
        Validator::validate_not_empty(&req.title, "Note title")?;
        Validator::validate_string_length(&req.title, "Note title", 1, 200)?;
        Validator::validate_not_empty(&req.content, "Note content")?;
        Validator::validate_date_format(&req.note_date)?;

        let title = Validator::sanitize_string(req.title);
        let content = req.content;
        let tags = Validator::sanitize_optional_string(req.tags);

        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();

        conn.execute(
            "INSERT INTO notes (title, content, note_date, tags, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            (&title, &content, &req.note_date, &tags, &now, &now),
        )?;

        let id = conn.last_insert_rowid();

        Ok(Note {
            id,
            title,
            content,
            note_date: req.note_date,
            tags,
            created_at: now.clone(),
            updated_at: now,
        })
    }

    pub fn get_notes_by_date(&self, date: &str) -> Result<Vec<Note>> {
        Validator::validate_date_format(date)?;

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
