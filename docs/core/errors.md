# Structured Error Handling & Versioned ErrorFiles

In MontRS, errors are not just strings; they are **data**. This document describes how the framework handles failures and how the versioned errorfile system works.

## 🧬 The `AiError` Trait

Every core error in MontRS implements the `AiError` trait, which provides:

-   `error_code`: A unique identifier for the error type (e.g., `E001`).
-   `explanation`: A detailed description of why the error occurred.
-   `suggested_fixes`: A list of actionable steps to resolve the issue.
-   `subsystem`: The part of the framework where the error originated.

## 📂 The `.llm/errorfiles` System

When the CLI encounters an error, it doesn't just print it to the terminal. It records it in a versioned history:

1.  **Detection**: An error occurs during a command (e.g., `montrs build`).
2.  **Recording**: A new `errorfile.json` is created in `.llm/errorfiles/vN/`.
3.  **Context**: The file includes the error data, timestamp, and a snapshot of the relevant code.
4.  **Resolution**: When the same command is run and succeeds, the system:
    -   Identifies the previously failed state.
    -   Generates a **diff** between the failing code and the fixed code.
    -   Stores the diff in the error directory as a "lesson learned."

## 🛠️ Practical Example: Implementing `AiError`

If you are building a custom package, here is how you should implement error handling:

```rust
use montrs::prelude::*;
use thiserror::Error;

#[derive(Error, Debug, AiError)]
pub enum MyPackageError {
    #[error("Database connection failed: {0}")]
    #[ai(code = "MP001", explanation = "Could not connect to the DB. Check your DATABASE_URL.")]
    DbConnection(String),
    
    #[error("Validation failed: {0}")]
    #[ai(code = "MP002", explanation = "Input data was malformed.")]
    Validation(String),
}
```

---

## 🔍 The Error Lifecycle

1.  **Failure**: `montrs build` fails with `E042`.
2.  **Snapshot**: `.llm/errorfiles/v1/E042_timestamp.json` is generated.
3.  **Fix**: The developer (or AI) modifies the code.
4.  **Verification**: `montrs build` runs again and succeeds.
5.  **Learning**: The CLI notices the fix, computes the diff, and saves it. Future AI agents can now see exactly how `E042` was resolved in this specific project context.
