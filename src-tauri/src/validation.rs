use anyhow::{Result, bail};

pub struct Validator;

impl Validator {
    pub fn validate_string_length(value: &str, field: &str, min: usize, max: usize) -> Result<()> {
        let len = value.trim().len();
        if len < min {
            bail!("{} must be at least {} characters", field, min);
        }
        if len > max {
            bail!("{} must be at most {} characters", field, max);
        }
        Ok(())
    }

    pub fn validate_not_empty(value: &str, field: &str) -> Result<()> {
        if value.trim().is_empty() {
            bail!("{} cannot be empty", field);
        }
        Ok(())
    }

    pub fn validate_email(email: &str) -> Result<()> {
        Self::validate_not_empty(email, "Email")?;
        if !email.contains('@') || !email.contains('.') {
            bail!("Invalid email format");
        }
        Ok(())
    }

    pub fn validate_language_code(code: &str) -> Result<()> {
        Self::validate_not_empty(code, "Language code")?;
        if code.len() < 2 || code.len() > 5 {
            bail!("Language code must be between 2 and 5 characters");
        }
        if !code.chars().all(|c| c.is_ascii_alphabetic() || c == '-') {
            bail!("Language code must contain only letters and hyphens");
        }
        Ok(())
    }

    pub fn validate_difficulty_level(level: i32) -> Result<()> {
        if !(1..=5).contains(&level) {
            bail!("Difficulty level must be between 1 and 5");
        }
        Ok(())
    }

    pub fn validate_quality_rating(quality: i32) -> Result<()> {
        if !(0..=5).contains(&quality) {
            bail!("Quality rating must be between 0 and 5");
        }
        Ok(())
    }

    pub fn validate_status(status: &str, allowed: &[&str]) -> Result<()> {
        if !allowed.contains(&status) {
            bail!("Invalid status. Allowed values: {}", allowed.join(", "));
        }
        Ok(())
    }

    pub fn validate_priority(priority: &str) -> Result<()> {
        let allowed = ["low", "medium", "high", "urgent"];
        Self::validate_status(priority, &allowed)
    }

    pub fn validate_date_format(date: &str) -> Result<()> {
        if date.is_empty() {
            return Ok(());
        }

        let parts: Vec<&str> = date.split('-').collect();
        if parts.len() != 3 {
            bail!("Date must be in YYYY-MM-DD format");
        }

        if parts[0].len() != 4 || parts[1].len() != 2 || parts[2].len() != 2 {
            bail!("Date must be in YYYY-MM-DD format");
        }

        Ok(())
    }

    pub fn sanitize_string(value: String) -> String {
        value.trim().to_string()
    }

    pub fn sanitize_optional_string(value: Option<String>) -> Option<String> {
        value.map(|s| Self::sanitize_string(s))
    }
}
