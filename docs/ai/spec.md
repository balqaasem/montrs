# The AI Specification Snapshot (`llm.json`)

The `llm.json` file, located in the `.llm/` directory, is the primary source of context for AI agents working on a MontRS project. It is a structured, machine-readable representation of the entire application.

## 📂 File Location
```text
.llm/
├── llm.json          # Primary JSON specification
├── llm.yaml          # YAML version (optional)
├── llm.txt           # Text summary (optional)
└── errorfiles/       # Versioned history of project errors
```

## 🏗️ Structure of `llm.json`

The file is divided into several key sections:

### 1. `metadata`
Contains project-wide information:
- `name`, `version`, `description`.
- AI-specific instructions and project context.

### 2. `modules`
A list of all registered `Module` implementations found in the project. For each module:
- `name`: The Rust struct name.
- `description`: The string returned by `module.description()`.

### 3. `routes`
The most critical section for API development. Each route includes:
- `path`: The URL pattern (e.g., `/api/users/:id`).
- `loader`: Metadata about the data-fetching component.
- `action`: Metadata about the mutation component.
- `input_schema` / `output_schema`: The expected data shapes.

### 4. `tools`
A curated list of CLI commands and framework capabilities that the AI can invoke as "tools."

## 🔄 Lifecycle

1. **Auto-Update**: The CLI automatically refreshes the spec whenever a command (`build`, `serve`, `test`) is run.
2. **Manual Update**: You can force a refresh using `montrs spec`.
3. **AI Consumption**: AI agents should read this file at the start of every session to ensure they have the latest context.

## 🤖 Why Not Just Read the Code?

While AI models *can* read source code, `llm.json` provides:
- **Pre-computed Metadata**: Descriptions and schemas are extracted and ready to use.
- **Unified View**: It bridges the gap between different files and packages.
- **Standardization**: It follows a consistent schema regardless of how the underlying code is structured.
