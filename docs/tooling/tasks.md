# Custom Tasks and Automation

MontRS provides a built-in task runner that replaces complex `Makefile`s or shell scripts. Tasks are defined in your `montrs.toml` file and can be executed via the CLI.

## 📝 Defining Tasks

Add a `[tasks]` section to your `montrs.toml`:

```toml
[tasks]
db-migrate = "sqlx migrate run"
db-seed = "cargo run --bin seed"
pre-commit = ["montrs fmt", "montrs test"]
```

## 🏃 Running Tasks

Use the `run` command followed by the task name:

```bash
montrs run db-migrate
```

## 🛠️ Task Features

- **String Tasks**: A simple shell command.
- **Array Tasks**: A sequence of tasks or commands run in order.
- **Environment Variables**: Tasks inherit the environment defined in `montrs.toml`.

## 🤖 AI and Tasks

AI agents can discover available tasks by reading the `montrs.toml` file or checking the `tools` section of `llm.json`. This allows an AI to perform complex operations like:
- "Run the database migrations before starting the server."
- "Execute the pre-commit checks to ensure the code is valid."

## 💡 Best Practices

1. **Self-Documenting Names**: Use descriptive names like `generate-assets` instead of `gen`.
2. **Keep Tasks Atomic**: A task should do one thing well. Use array tasks to compose them.
3. **Use MontRS Commands**: Prefer `montrs fmt` over `rustfmt` to ensure the project-specific formatting rules are applied.
