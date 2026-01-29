# AI Guide: montrs-schema

This guide helps AI models use the declarative validation system of MontRS.

## Core Concepts

### 1. `#[derive(Schema)]`
The primary macro for defining validation rules. It generates a `validate(&self) -> Result<(), Vec<ValidationError>>` method.

### 2. Validation Attributes
- `min_len = N`: Validates string length.
- `email`: Validates email format.
- `regex = "..."`: Validates against a regular expression.
- `custom = "method"`: Delegates to a custom method returning `Result<(), String>`.

## AI Usage Patterns

### Defining a Validated Struct
When generating data models, always include validation attributes to ensure data integrity.
```rust
#[derive(Schema, Serialize, Deserialize)]
pub struct ProjectConfig {
    #[schema(min_len = 3)]
    pub name: String,
    #[schema(email)]
    pub contact_email: String,
}
```

### Handling Validation Errors
If `validate()` returns an error, it will contain a list of `ValidationError` variants. Use these to prompt the user or self-correct the input data.
- `MinLength { field, min, actual }`
- `InvalidEmail { field }`
- `RegexMismatch { field, pattern }`
- `Custom { field, message }`
