use crate::models::*;
use crate::db::Database;
use crate::validation::*;
use anyhow::Result;
use chrono::Utc;

impl Database {
    pub fn create_project(&self, req: CreateProjectRequest) -> Result<Project> {
validate_not_empty(&req.name, "Project name")?;
validate_string_length(&req.name, "Project name", 1, 200)?;
validate_status(&req.status, &["active", "completed", "on_hold", "cancelled"])?;
validate_priority(&req.priority)?;

        if let Some(ref start_date) = req.start_date {
validate_date_format(start_date)?;
        }
        if let Some(ref end_date) = req.end_date {
validate_date_format(end_date)?;
        }

        let name = sanitize_string(req.name);
        let description = sanitize_optional_string(req.description);
        let status = sanitize_string(req.status);
        let priority = sanitize_string(req.priority);

        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();

        conn.execute(
            "INSERT INTO projects (name, description, status, priority, start_date, end_date, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            (&name, &description, &status, &priority, &req.start_date, &req.end_date, &now, &now),
        )?;

        let id = conn.last_insert_rowid();

        Ok(Project {
            id,
            name,
            description,
            status,
            priority,
            start_date: req.start_date,
            end_date: req.end_date,
            created_at: now.clone(),
            updated_at: now,
        })
    }

    pub fn get_projects(&self) -> Result<Vec<Project>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, description, status, priority, start_date, end_date, created_at, updated_at
             FROM projects ORDER BY updated_at DESC"
        )?;

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

    pub fn create_task(&self, req: CreateTaskRequest) -> Result<Task> {
validate_not_empty(&req.title, "Task title")?;
validate_string_length(&req.title, "Task title", 1, 200)?;
validate_status(&req.status, &["todo", "in_progress", "completed", "blocked"])?;
validate_priority(&req.priority)?;

        if let Some(ref due_date) = req.due_date {
validate_date_format(due_date)?;
        }

        let title = sanitize_string(req.title);
        let description = sanitize_optional_string(req.description);
        let status = sanitize_string(req.status);
        let priority = sanitize_string(req.priority);

        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();

        conn.execute(
            "INSERT INTO tasks (project_id, title, description, status, priority, due_date, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            (&req.project_id, &title, &description, &status, &priority, &req.due_date, &now, &now),
        )?;

        let id = conn.last_insert_rowid();

        Ok(Task {
            id,
            project_id: req.project_id,
            title,
            description,
            status,
            priority,
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
validate_status(status, &["todo", "in_progress", "completed", "blocked"])?;

        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();
        let completed_at = if status == "completed" { Some(now.clone()) } else { None };

        conn.execute(
            "UPDATE tasks SET status = ?1, completed_at = ?2, updated_at = ?3 WHERE id = ?4",
            (status, &completed_at, &now, task_id),
        )?;

        Ok(())
    }
}
