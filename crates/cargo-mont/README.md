# cargo-mont

The official build and orchestration tool for the MontRS framework.

## Overview

`cargo-mont` is a specialized cargo subcommand designed to manage the lifecycle of MontRS applications. It provides high-level commands for scaffolding, building, and serving projects, integrating deeply with `trunk` and `cargo`.

## Key Features

- **Cargo Subcommand**: Seamlessly integrates into the Rust ecosystem as `cargo mont`.
- **Sophisticated Scaffolding**: Create new projects from local templates via `cargo mont new`.
- **Integrated Build System**: Orchestrates both frontend (`trunk`) and server-side (`cargo`) builds with a single command: `cargo mont build`.
- **Developer Experience**: Parallelized development server with hot-reloading via `cargo mont serve`.
- **Local Template Management**: Templates are managed within the monorepo for maximum stability and speed.

## Installation

```bash
cargo install --path crates/cargo-mont
```

## Usage

### Create a New Project
```bash
cargo mont new my-app
```

### Build for Production
```bash
cargo mont build
```

### Start Development Server
```bash
cargo mont serve
```
