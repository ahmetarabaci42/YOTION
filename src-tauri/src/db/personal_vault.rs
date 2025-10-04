use crate::models::*;
use crate::db::Database;
use crate::validation::Validator;
use crate::encryption::EncryptedData;
use anyhow::Result;
use chrono::Utc;

impl Database {
    pub fn create_personal_account(&self, req: CreatePersonalAccountRequest) -> Result<PersonalAccount> {
        Validator::validate_not_empty(&req.title, "Account title")?;
        Validator::validate_string_length(&req.title, "Account title", 1, 200)?;
        Validator::validate_email(&req.email)?;
        Validator::validate_not_empty(&req.password, "Password")?;

        let title = Validator::sanitize_string(req.title);
        let email = Validator::sanitize_string(req.email);
        let encrypted_password = EncryptedData::new(&req.password)?;
        let website = Validator::sanitize_optional_string(req.website);
        let notes = Validator::sanitize_optional_string(req.notes);
        let category = Validator::sanitize_string(req.category);

        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();

        conn.execute(
            "INSERT INTO personal_accounts (title, email, password, website, notes, category, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            (&title, &email, encrypted_password.as_bytes(), &website, &notes, &category, &now, &now),
        )?;

        let id = conn.last_insert_rowid();

        Ok(PersonalAccount {
            id,
            title,
            email,
            password: encrypted_password.to_string(),
            website,
            notes,
            category,
            created_at: now.clone(),
            updated_at: now,
        })
    }

    pub fn get_personal_accounts(&self) -> Result<Vec<PersonalAccount>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, title, email, password, website, notes, category, created_at, updated_at
             FROM personal_accounts ORDER BY category, title"
        )?;

        let account_iter = stmt.query_map([], |row| {
            Ok(PersonalAccount {
                id: row.get(0)?,
                title: row.get(1)?,
                email: row.get(2)?,
                password: row.get(3)?,
                website: row.get(4)?,
                notes: row.get(5)?,
                category: row.get(6)?,
                created_at: row.get(7)?,
                updated_at: row.get(8)?,
            })
        })?;

        let mut accounts = Vec::new();
        for account in account_iter {
            accounts.push(account?);
        }

        Ok(accounts)
    }

    pub fn get_personal_accounts_by_category(&self, category: &str) -> Result<Vec<PersonalAccount>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, title, email, password, website, notes, category, created_at, updated_at
             FROM personal_accounts WHERE category = ?1 ORDER BY title"
        )?;

        let account_iter = stmt.query_map([category], |row| {
            Ok(PersonalAccount {
                id: row.get(0)?,
                title: row.get(1)?,
                email: row.get(2)?,
                password: row.get(3)?,
                website: row.get(4)?,
                notes: row.get(5)?,
                category: row.get(6)?,
                created_at: row.get(7)?,
                updated_at: row.get(8)?,
            })
        })?;

        let mut accounts = Vec::new();
        for account in account_iter {
            accounts.push(account?);
        }

        Ok(accounts)
    }

    pub fn create_personal_info(&self, req: CreatePersonalInfoRequest) -> Result<PersonalInfo> {
        Validator::validate_not_empty(&req.title, "Info title")?;
        Validator::validate_string_length(&req.title, "Info title", 1, 200)?;
        Validator::validate_not_empty(&req.content, "Content")?;

        let title = Validator::sanitize_string(req.title);
        let content = if req.is_sensitive {
            EncryptedData::new(&req.content)?.to_string()
        } else {
            req.content
        };
        let category = Validator::sanitize_string(req.category);

        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();

        conn.execute(
            "INSERT INTO personal_info (title, content, category, is_sensitive, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            (&title, &content, &category, &req.is_sensitive, &now, &now),
        )?;

        let id = conn.last_insert_rowid();

        Ok(PersonalInfo {
            id,
            title,
            content,
            category,
            is_sensitive: req.is_sensitive,
            created_at: now.clone(),
            updated_at: now,
        })
    }

    pub fn get_personal_info(&self) -> Result<Vec<PersonalInfo>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, title, content, category, is_sensitive, created_at, updated_at
             FROM personal_info ORDER BY category, title"
        )?;

        let info_iter = stmt.query_map([], |row| {
            Ok(PersonalInfo {
                id: row.get(0)?,
                title: row.get(1)?,
                content: row.get(2)?,
                category: row.get(3)?,
                is_sensitive: row.get(4)?,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
            })
        })?;

        let mut info_list = Vec::new();
        for info in info_iter {
            info_list.push(info?);
        }

        Ok(info_list)
    }

    pub fn get_personal_info_by_category(&self, category: &str) -> Result<Vec<PersonalInfo>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, title, content, category, is_sensitive, created_at, updated_at
             FROM personal_info WHERE category = ?1 ORDER BY title"
        )?;

        let info_iter = stmt.query_map([category], |row| {
            Ok(PersonalInfo {
                id: row.get(0)?,
                title: row.get(1)?,
                content: row.get(2)?,
                category: row.get(3)?,
                is_sensitive: row.get(4)?,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
            })
        })?;

        let mut info_list = Vec::new();
        for info in info_iter {
            info_list.push(info?);
        }

        Ok(info_list)
    }
}
