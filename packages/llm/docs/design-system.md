# MontRS AI Messaging Design System

This document defines the structured way MontRS articulates information, documentation, and errors to AI models.

## 1. Tone and Articulation

### Human-Facing Tone
- **Professional & Helpful**: Concise instructions, clear warnings.
- **Action-Oriented**: Focus on what the user needs to do next.

### AI-Facing Articulation (The "Model-First" Layer)
- **Deterministic**: Use stable identifiers (error codes, trait names).
- **Context-Rich**: Always include surrounding context (code snippets, file paths, line numbers).
- **Remediation-Focused**: Provide "suggested_fixes" that are machine-actionable.

## 2. Metadata Structure

All framework components (Modules, Loaders, Actions) must expose metadata via the following schema:

| Field | Type | Description |
|-------|------|-------------|
| `description` | String | A high-level summary of the component's purpose. |
| `input_schema` | JSON | (Optional) Validated schema for input data. |
| `output_schema`| JSON | (Optional) Validated schema for output data. |
| `tags` | Array | Categorical tags (e.g., "auth", "db", "ui"). |

## 3. Error Articulation (AiError)

Errors are the primary communication channel during failure. Every `AiError` must follow this structure:

```json
{
  "error_code": "STABLE_ID_UPPERCASE",
  "explanation": "A detailed explanation of the cause.",
  "suggested_fixes": [
    "Step-by-step fix 1",
    "Step-by-step fix 2"
  ],
  "subsystem": "package-name",
  "rustc_error": "Optional raw compiler output"
}
```

## 4. Documentation Mapping

Documentation is curated into the `.llm/llm.json` snapshot using these priorities:
1. **Explicit Metadata**: Values returned by `metadata()` methods.
2. **Doc Comments**: Triple-slash `///` comments on public items.
3. **AI Guides**: Markdown files in `/docs/ai-guide.md`.
4. **READMEs**: Key sections (Key Features, Usage).

## 5. Learning from Resolution

When an error is resolved, the CLI generates a diff. This diff is structured as:
- **Base**: The error record and the state of the code at the time of failure.
- **Fix**: The commit/changes that resolved the error.
- **Lesson**: A summary of why the fix worked.

AI models should prioritize these "Resolution Diffs" as the most reliable source of truth for debugging.
