use crate::models::*;
use crate::db::Database;
use crate::validation::Validator;
use anyhow::Result;
use chrono::Utc;

impl Database {
    pub fn create_language(&self, req: CreateLanguageRequest) -> Result<Language> {
        Validator::validate_not_empty(&req.name, "Language name")?;
        Validator::validate_string_length(&req.name, "Language name", 1, 50)?;
        Validator::validate_language_code(&req.code)?;
        Validator::validate_not_empty(&req.flag_emoji, "Flag emoji")?;

        let name = Validator::sanitize_string(req.name);
        let code = Validator::sanitize_string(req.code);
        let flag_emoji = Validator::sanitize_string(req.flag_emoji);

        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();

        conn.execute(
            "INSERT INTO languages (name, code, flag_emoji, created_at) VALUES (?1, ?2, ?3, ?4)",
            (&name, &code, &flag_emoji, &now),
        )?;

        let id = conn.last_insert_rowid();

        Ok(Language {
            id,
            name,
            code,
            flag_emoji,
            created_at: now,
        })
    }

    pub fn get_languages(&self) -> Result<Vec<Language>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, code, flag_emoji, created_at FROM languages ORDER BY created_at DESC"
        )?;

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
}
