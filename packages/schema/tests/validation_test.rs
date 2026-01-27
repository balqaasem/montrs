use montrs_core::{Validate, ValidationError};
use montrs_schema::Schema;

#[derive(Schema)]
struct User {
    #[schema(min_len = 3)]
    username: String,
    #[schema(email)]
    email: String,
    #[schema(regex = r"^\d{4}-\d{2}-\d{2}$")]
    birth_date: String,
    #[schema(custom = "validate_custom")]
    status: String,
}

impl User {
    fn validate_custom(&self) -> Result<(), String> {
        if self.status == "forbidden" {
            Err("Status cannot be forbidden".to_string())
        } else {
            Ok(())
        }
    }
}

#[test]
fn test_validation_success() {
    let user = User {
        username: "alice".to_string(),
        email: "alice@example.com".to_string(),
        birth_date: "1990-01-01".to_string(),
        status: "active".to_string(),
    };

    assert!(user.validate().is_ok());
}

#[test]
fn test_validation_failure_multiple_errors() {
    let user = User {
        username: "al".to_string(),         // too short
        email: "invalid-email".to_string(), // no @
        birth_date: "90-01-01".to_string(), // wrong format
        status: "forbidden".to_string(),    // custom error
    };

    let result = user.validate();
    assert!(result.is_err());
    let errors = result.unwrap_err();

    assert_eq!(errors.len(), 4);

    assert!(matches!(
        errors[0],
        ValidationError::MinLength {
            field: "username",
            min: 3,
            actual: 2
        }
    ));
    assert!(matches!(
        errors[1],
        ValidationError::InvalidEmail { field: "email" }
    ));
    assert!(matches!(
        errors[2],
        ValidationError::RegexMismatch {
            field: "birth_date",
            ..
        }
    ));
    assert!(matches!(
        errors[3],
        ValidationError::Custom {
            field: "status",
            ..
        }
    ));
}

#[test]
fn test_regex_lazy_initialization() {
    let user = User {
        username: "bob".to_string(),
        email: "bob@example.com".to_string(),
        birth_date: "2000-12-31".to_string(),
        status: "active".to_string(),
    };

    // First call initializes regex
    assert!(user.validate().is_ok());
    // Second call uses already initialized regex
    assert!(user.validate().is_ok());
}
