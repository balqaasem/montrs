# Agent Guide: montrs-cli

This guide helps agents interact with the MontRS command-line interface.

## Core Concepts

### 1. Project Management
- `montrs new <name>`: Scaffolds a new project.
- `montrs build`: Compiles the project.
- `montrs serve`: Starts the dev server with hot-reload.

### 2. Agent-first commands
- `montrs spec`: Generates an agent-readable snapshot of the project in `.agent/agent.json`. This is the primary way for an agent to "see" the project structure and routing.
- **Auto-generation**: The CLI automatically updates `.agent/agent.json` and `.agent/tools.json` on almost every interaction.

### 3. Diagnostics and Debugging
- If a command fails, the CLI generates a versioned error record in `.agent/errorfiles/vN/<id>.json`.
- Agents should read these files to understand the context and suggested fixes.

## Agent Usage Patterns

### Getting a Project Overview
Before making changes, always suggest running `montrs spec` or reading `.agent/agent.json` to understand the current state.

### Resolving Errors
When a build fails, look for the latest entry in `.agent/errorfiles/`. It contains:
- `error_code`: A stable ID for the error type.
- `explanation`: Why it failed.
- `suggested_fixes`: Actionable steps to fix it.
- `rustc_error`: The raw compiler output if applicable.

### Using Tools
Refer to `.agent/tools.json` for a list of available tools (functions) provided by the MontRS ecosystem.
