use crate::errors::AppError;

// Trait for validation
pub trait Validator {
    fn validate(&self) -> Result<(), AppError>;
}

// Validation functions
pub fn validate_not_empty(value: &str, field_name: &str) -> Result<(), AppError> {
    if value.trim().is_empty() {
        return Err(AppError::Validation(format!("{} cannot be empty", field_name)));
    }
    Ok(())
}

pub fn validate_string_length(value: &str, field_name: &str, min: usize, max: usize) -> Result<(), AppError> {
    let len = value.len();
    if len < min || len > max {
        return Err(AppError::Validation(format!("{} must be between {} and {} characters", field_name, min, max)));
    }
    Ok(())
}

pub fn validate_difficulty_level(level: i64) -> Result<(), AppError> {
    if level < 1 || level > 5 {
        return Err(AppError::Validation("Difficulty level must be between 1 and 5".to_string()));
    }
    Ok(())
}

pub fn validate_quality_rating(rating: i64) -> Result<(), AppError> {
    if rating < 1 || rating > 5 {
        return Err(AppError::Validation("Quality rating must be between 1 and 5".to_string()));
    }
    Ok(())
}

pub fn validate_date_format(date: &str) -> Result<(), AppError> {
    // Simple date format validation - you can make this more sophisticated
    if date.len() != 10 || !date.chars().all(|c| c.is_ascii_digit() || c == '-') {
        return Err(AppError::Validation("Date must be in YYYY-MM-DD format".to_string()));
    }
    Ok(())
}

pub fn validate_status(value: &str, allowed_values: &[&str]) -> Result<(), AppError> {
    if !allowed_values.contains(&value) {
        return Err(AppError::Validation(format!("Status must be one of: {}", allowed_values.join(", "))));
    }
    Ok(())
}

// Sanitization functions
pub fn sanitize_string(value: String) -> String {
    value.trim().to_string()
}

pub fn sanitize_optional_string(value: Option<String>) -> Option<String> {
    value.map(|v| v.trim().to_string())
}

pub fn validate_language_code(code: &str) -> Result<(), AppError> {
    if code.len() != 2 {
        return Err(AppError::Validation("Language code must be exactly 2 characters".to_string()));
    }
    if !code.chars().all(|c| c.is_ascii_lowercase()) {
        return Err(AppError::Validation("Language code must be lowercase letters".to_string()));
    }
    Ok(())
}

pub fn validate_email(email: &str) -> Result<(), AppError> {
    if !email.contains('@') || !email.contains('.') {
        return Err(AppError::Validation("Invalid email format".to_string()));
    }
    Ok(())
}

pub fn validate_password_strength(password: &str) -> Result<(), AppError> {
    if password.len() < 8 {
        return Err(AppError::Validation("Password must be at least 8 characters long".to_string()));
    }
    Ok(())
}

pub fn validate_priority(priority: &str) -> Result<(), AppError> {
    match priority {
        "low" | "medium" | "high" => Ok(()),
        _ => Err(AppError::Validation("Priority must be 'low', 'medium', or 'high'".to_string())),
    }
}
