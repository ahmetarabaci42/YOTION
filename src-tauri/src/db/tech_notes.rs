use crate::models::*;
use crate::db::Database;
use crate::validation::Validator;
use anyhow::Result;
use chrono::Utc;

impl Database {
    pub fn create_tech_space(&self, req: CreateTechSpaceRequest) -> Result<TechSpace> {
        Validator::validate_not_empty(&req.name, "Tech space name")?;
        Validator::validate_string_length(&req.name, "Tech space name", 1, 100)?;
        Validator::validate_not_empty(&req.icon, "Icon")?;

        let name = Validator::sanitize_string(req.name);
        let description = Validator::sanitize_optional_string(req.description);
        let icon = Validator::sanitize_string(req.icon);

        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();

        conn.execute(
            "INSERT INTO tech_spaces (name, description, icon, created_at) VALUES (?1, ?2, ?3, ?4)",
            (&name, &description, &icon, &now),
        )?;

        let id = conn.last_insert_rowid();

        Ok(TechSpace {
            id,
            name,
            description,
            icon,
            created_at: now,
        })
    }

    pub fn get_tech_spaces(&self) -> Result<Vec<TechSpace>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, description, icon, created_at FROM tech_spaces ORDER BY created_at DESC"
        )?;

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

    pub fn create_code_snippet(&self, req: CreateCodeSnippetRequest) -> Result<CodeSnippet> {
        Validator::validate_not_empty(&req.title, "Title")?;
        Validator::validate_string_length(&req.title, "Title", 1, 200)?;
        Validator::validate_not_empty(&req.code, "Code")?;
        Validator::validate_not_empty(&req.language, "Language")?;

        let title = Validator::sanitize_string(req.title);
        let description = Validator::sanitize_optional_string(req.description);
        let code = req.code;
        let language = Validator::sanitize_string(req.language);
        let tags = Validator::sanitize_optional_string(req.tags);

        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();

        conn.execute(
            "INSERT INTO code_snippets (tech_space_id, title, description, code, language, tags, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            (&req.tech_space_id, &title, &description, &code, &language, &tags, &now, &now),
        )?;

        let id = conn.last_insert_rowid();

        Ok(CodeSnippet {
            id,
            tech_space_id: req.tech_space_id,
            title,
            description,
            code,
            language,
            tags,
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
}
