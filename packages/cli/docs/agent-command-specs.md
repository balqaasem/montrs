# MontRS Agent Command Specifications

This document defines the behavior and invariants for the `montrs agent` subcommands.

---

## 1. `montrs agent check`

**Purpose**: Validate structural correctness and catch obvious agent-hostile patterns.

### Inputs
- `--path`: Directory to check (default: current).
- `--level`: Strictness level (relaxed, standard, strict).

### Invariants
- Must be **read-only**.
- Must be **fast** (sub-second for most projects).
- Must validate:
  - Valid `montrs.toml`.
  - Package dependency cycles.
  - Correct `Plate` and `Route` trait implementations (structural check).
  - Presence of required Scaffolded Explicit headers.

### Output
- **Human**: Success message or a list of structural violations.
- **AI**: JSON object with `valid: bool` and a list of `violations` including file paths and line numbers.

---

## 2. `montrs agent doctor`

**Purpose**: Assess if the project is "safe and understandable" for an AI agent.

### Inputs
- `--package`: Optional package name to focus on.

### Responsibilities
- Verifies documentation coverage for public traits.
- Checks for `@agent-tool` marker consistency.
- Detects "magic" or highly coupled patterns that lack explicit interfaces.
- Analyzes error history (if available) to find recurring confusion points.

### Output
- **Human**: A "health report" with explanations of WHY certain patterns are problematic.
- **AI**: Structured diagnostics including `severity` (info, warning, error), `invariant_violated`, and `suggested_remediation`.

---

## 3. `montrs agent diff`

**Purpose**: Show a diagnostic diff for an error file, including the error description, the code that produced it, and the suggested fix with the fixed code.

### Inputs
- `--path`: Path to the error file or diagnostic report.

### Design Expectations
- Identifies the specific code block causing the error.
- Displays the error message and context.
- Provides a "Before" vs "After" diff showing the suggested fix.

### Output
- **Human**: A clear, color-coded diff showing the error, offending code, and proposed fix.
- **AI**: A JSON structure containing the `error`, `original_code`, and `fixed_code`.
