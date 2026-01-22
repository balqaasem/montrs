use std::fmt;

/// Errors that can occur during schema validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    /// Field length is less than the required minimum.
    MinLength { field: &'static str, min: usize, actual: usize },
    /// Field does not contain a valid email format.
    InvalidEmail { field: &'static str },
    /// Field does not match the required regular expression.
    RegexMismatch { field: &'static str, pattern: &'static str },
    /// A custom validation rule failed.
    Custom { field: &'static str, message: String },
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::MinLength { field, min, actual } => {
                write!(f, "{} is too short: {} (min {})", field, actual, min)
            }
            ValidationError::InvalidEmail { field } => {
                write!(f, "{} must be a valid email", field)
            }
            ValidationError::RegexMismatch { field, pattern } => {
                write!(f, "{} does not match pattern: {}", field, pattern)
            }
            ValidationError::Custom { field, message } => {
                write!(f, "{}: {}", field, message)
            }
        }
    }
}

impl std::error::Error for ValidationError {}

/// Trait for types that can be validated against a schema.
pub trait Validate {
    /// Validates the struct and returns a list of all validation errors found.
    fn validate(&self) -> Result<(), Vec<ValidationError>>;
}
