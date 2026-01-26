# cargo-mont

The official build and orchestration tool for the MontRS framework.

## Overview

`cargo-mont` is a specialized cargo subcommand designed to manage the lifecycle of MontRS applications. It provides high-level commands for scaffolding, building, and serving projects, acting as the primary orchestration hub for the framework.

## Key Features

- **Cargo Subcommand**: Seamlessly integrates into the Rust ecosystem as `cargo mont`.
- **Sophisticated Scaffolding**: Create new projects from local templates via `cargo mont new`.
- **Integrated Build System**: Orchestrates both frontend and server-side builds.
- **Advanced Task Runner**: Define custom shell scripts with dependencies in `mont.toml` and run them via `cargo mont run`.
- **Unified Configuration**: Manage ports, addresses, and target directories for both frontend and backend in one place.
- **Leptos Ready**: Automatically injects environment variables (`LEPTOS_SITE_ROOT`, etc.) for seamless server execution.

## Installation

### From Crates.io

```bash
cargo install cargo-mont
```

### From Source

```bash
cargo install --path packages/cargo-mont
```

## Usage

### Create a New Project
```bash
cargo mont new my-app
```

### Run Custom Tasks
```bash
cargo mont tasks        # List all tasks
cargo mont run lint     # Run the 'lint' task
```

### Build for Production
```bash
cargo mont build
```

### Start Development Server
```bash
cargo mont serve
```

### Benchmarking
Quickly measure execution speed and file size:
```bash
# Benchmark a project (standard cargo bench)
cargo mont bench

# Simple/Native benchmark for a single file/binary
cargo mont bench --simple ./main.rs --iterations 500
```

## Configuration (`mont.toml`)

`cargo-mont` honors a `mont.toml` file in your project root for extensive configuration:

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
