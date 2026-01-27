# cargo-montrs

The official build and orchestration tool for the MontRS framework.

## Overview

`cargo-montrs` is a specialized cargo subcommand designed to manage the lifecycle of MontRS applications. It provides high-level commands for scaffolding, building, and serving projects, acting as the primary orchestration hub for the framework.

## Key Features

- **Cargo Subcommand**: Seamlessly integrates into the Rust ecosystem as `cargo montrs`.
- **Sophisticated Scaffolding**: Create new projects from local templates via `cargo montrs new`.
- **Integrated Build System**: Orchestrates both frontend and server-side builds.
- **Advanced Task Runner**: Define custom shell scripts with dependencies in `montrs.toml` and run them via `cargo montrs run`.
- **Unified Configuration**: Manage ports, addresses, and target directories for both frontend and backend in one place.
- **Leptos Ready**: Automatically injects environment variables (`LEPTOS_SITE_ROOT`, etc.) for seamless server execution.

## Installation

### From Crates.io

```bash
cargo install cargo-montrs
```

### From Source

```bash
cargo install --path packages/cargo-montrs
```

## Usage

### Create a New Project
```bash
cargo montrs new my-app
```

### Run Custom Tasks
```bash
cargo montrs tasks        # List all tasks
cargo montrs run lint     # Run the 'lint' task
```

### Build for Production
```bash
cargo montrs build
```

### Start Development Server
```bash
cargo montrs serve
```

### Benchmarking
Quickly measure execution speed and file size:
```bash
# Benchmark a project (standard cargo bench)
cargo montrs bench

# Simple/Native benchmark for a single file/binary
cargo montrs bench --simple ./main.rs --iterations 500
```

## Configuration (`montrs.toml`)

`cargo-montrs` honors a `montrs.toml` file in your project root for extensive configuration:

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
