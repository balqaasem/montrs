# cargo-mont CLI Reference

The `cargo-mont` CLI is the primary tool for managing MontRS applications.

## General Flags

- `--release`: Build artifacts in release mode.
- `--hot-reload`: Enable partial hot-reloading.
- `--verbose`, `-v`: Increase logging verbosity.
- `--features`: Specify features to use during compilation.

## Commands

### `new`
Scaffold a new MontRS project from a template.
```bash
cargo mont new <name> [--template <template>]
```

### `build`
Build the project for production.
```bash
cargo mont build
```

### `serve`
Start the development server with hot-reloading.
```bash
cargo mont serve
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
cargo mont bench

# Native benchmark of a script
cargo mont bench --simple scripts/process_data.rs
```

### `test`
Run project tests (Unit, Integration, E2E).
```bash
cargo mont test [--filter <name>] [--report <format>]
```

### `run`
Run custom tasks defined in `mont.toml`.
```bash
cargo mont run <task_name>
```

### `watch`
Watch for changes and rebuild automatically.
```bash
cargo mont watch
```
