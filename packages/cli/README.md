# montrs-cli

The official build and orchestration tool for the MontRS framework.

## Overview

`montrs-cli` is the primary orchestration hub for the MontRS framework. It provides high-level commands for scaffolding, building, and serving projects. It can be used as a standalone `montrs` command or as a cargo subcommand (`cargo montrs`).

## Key Features

- **Standalone & Cargo Subcommand**: Use it directly as `montrs` or via `cargo montrs`.
- **Sophisticated Scaffolding**: Create new projects from local templates via `montrs new`.
- **Integrated Build System**: Orchestrates both frontend and server-side builds.
- **Advanced Task Runner**: Define custom shell scripts with dependencies in `montrs.toml` and run them via `montrs run`.
- **Unified Configuration**: Manage ports, addresses, and target directories for both frontend and backend in one place.
- **Leptos Ready**: Automatically injects environment variables (`LEPTOS_SITE_ROOT`, etc.) for seamless server execution.

## Installation

### Recommended (via Meta-crate)

```bash
cargo install montrs
```

### Direct CLI Installation

```bash
cargo install montrs-cli
```

### From Source

```bash
cargo install --path packages/cli
```

## Usage

### Create a New Project
```bash
montrs new my-app
```

### Run Custom Tasks
```bash
montrs tasks        # List all tasks
montrs run lint     # Run the 'lint' task
```

### Build for Production
```bash
montrs build
```

### Start Development Server
```bash
montrs serve
```

### Benchmarking
Quickly measure execution speed and file size:
```bash
# Benchmark a project (standard cargo bench)
montrs bench

# Simple/Native benchmark for a single file/binary
montrs bench --simple ./main.rs --iterations 500
```

## Configuration (`montrs.toml`)

`montrs` honors a `montrs.toml` file in your project root for extensive configuration:

```toml
[project]
name = "my-app"

[build]
target = "index.html"
dist = "dist"

[serve]
port = 8080
addr = "127.0.0.1"

[tasks]
lint = "cargo clippy"
test-all = { command = "cargo test", dependencies = ["lint"] }
```
