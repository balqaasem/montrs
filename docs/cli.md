# montrs CLI Reference

The `montrs` CLI is the primary tool for managing MontRS applications.

## General Flags

- `--release`: Build artifacts in release mode.
- `--hot-reload`: Enable partial hot-reloading.
- `--verbose`, `-v`: Increase logging verbosity.
- `--features`: Specify features to use during compilation.

## Commands

### `new`
Scaffold a new MontRS project from a template.
```bash
montrs new <name> [--template <template>]
```

### `build`
Build the project for production.
```bash
montrs build
```

### `serve`
Start the development server with hot-reloading.
```bash
montrs serve
```

### `bench`
Run performance benchmarks.

**Arguments:**
- `target`: (Optional) The target file, directory, or filter.

**Flags:**
- `--iterations <N>`: Number of benchmark iterations (default: 100).
- `--warmup <N>`: Number of warmup runs (default: 10).
- `--timeout <SECONDS>`: Maximum execution time for each benchmark.
- `--filter <STRING>`: Filter benchmarks by name.
- `--json-output <PATH>`: Export results to a JSON file.
- `--generate-weights <PATH>`: Generate a Rust file with weight constants (Substrate-style).
- `--simple`: **Native Mode**. Benchmarks a file/binary directly without project overhead. Requires `target`.

**Examples:**
```bash
# Standard project benchmarks
montrs bench

# Native benchmark of a script
montrs bench --simple scripts/process_data.rs
```

### `fmt`
Format the project's Rust and view! code.

**Arguments:**
- `path`: (Optional) The file or directory to format (default: `.`).

**Flags:**
- `--check`: Verifies if files are formatted without modifying them.
- `--verbose`: Show detailed output.

**Examples:**
```bash
# Format everything
montrs fmt

# Check formatting in CI
montrs fmt --check
```

### `test`
Run project tests (Unit, Integration, E2E).
```bash
montrs test [--filter <name>] [--report <format>]
```

### `run`
Run custom tasks defined in `montrs.toml`.
```bash
montrs run <task_name>
```

### `watch`
Watch for changes and rebuild automatically.
```bash
montrs watch
```

### `upgrade`
Upgrade the montrs CLI to the latest version.
```bash
montrs upgrade
```
