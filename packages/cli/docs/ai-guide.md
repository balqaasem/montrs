# AI Guide: montrs-cli

This guide helps AI models interact with the MontRS command-line interface.

## Core Concepts

### 1. Project Management
- `montrs new <name>`: Scaffolds a new project.
- `montrs build`: Compiles the project.
- `montrs serve`: Starts the dev server with hot-reload.

### 2. AI-First Commands
- `montrs spec`: Generates an AI-readable snapshot of the project in `.llm/llm.json`. This is the primary way for an AI to "see" the project structure and routing.
- **Auto-generation**: The CLI automatically updates `.llm/llm.json` and `.llm/tools.json` on almost every interaction.

### 3. Diagnostics and Debugging
- If a command fails, the CLI generates a versioned error record in `.llm/errorfiles/vN/<id>.json`.
- AI models should read these files to understand the context and suggested fixes.

## AI Usage Patterns

### Getting a Project Overview
Before making changes, always suggest running `montrs spec` or reading `.llm/llm.json` to understand the current state.

### Resolving Errors
When a build fails, look for the latest entry in `.llm/errorfiles/`. It contains:
- `error_code`: A stable ID for the error type.
- `explanation`: Why it failed.
- `suggested_fixes`: Actionable steps to fix it.
- `rustc_error`: The raw compiler output if applicable.

### Using Tools
Refer to `.llm/tools.json` for a list of available AI tools (functions) provided by the MontRS ecosystem.
