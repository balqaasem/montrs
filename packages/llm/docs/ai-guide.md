# AI Usage Guide for montrs-llm

This document explains how AI models should interact with the `montrs-llm` package and the files it generates.

## The `.llm` Super Folder

The root `.llm` folder is the primary interface for AI models. It contains:

### `llm.json`
The project snapshot. It includes:
- `structure`: A flat list of files in the project (excluding `.git`, `target`, etc.).
- `modules`: High-level summaries of all MontRS modules found in the app.
- `routes`: All registered loaders and actions with their input/output schemas.
- `documentation_snippets`: Relevant guides (architecture, debugging) for quick context.

### `tools.json`
Definitions of available CLI tools as function calls. AI models should use these to execute commands on behalf of the user.

### `errorfiles/`
A versioned repository of errors.
- `v1/`, `v2/`, etc.: Directories representing the version of the error record.
- `<error-id>.json`: A structured record of a specific error.

## Error Handling Flow

When an AI model encounters an error (e.g., a build failure reported by the CLI):

1. **Read the Error**: Look for the latest entry in `.llm/errorfiles/`.
2. **Analyze Context**: The `detail` field provides the file, line, column, and code context.
3. **Check History**: If the error has a history, look at previous versions to see if similar fixes were attempted.
4. **Propose Fix**: Based on the `ai_metadata` (if available) and the code context, propose a fix.
5. **Verify**: Run the build/test command again.
6. **Record Resolution**: Once fixed, the CLI will call `resolve_error` to create a new version with the diff.

## Best Practices for AI

- **Prefer `llm.json` over scanning the whole disk**: It's faster and contains pre-filtered information.
- **Use the `subsystem` metadata**: When an error occurs, the `subsystem` field in `AiError` metadata helps pinpoint which part of the framework is complaining.
- **Leverage `suggested_fixes`**: These are human-authored hints specifically for resolving common issues in that subsystem.
