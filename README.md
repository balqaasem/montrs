<!-- MontRS: The Rust-native, trait-driven deterministic web framework. -->

# MontRS

**MontRS** is a Full-Stack Framework for Cross-Platform Rust Applications (A Leptos Framework). **MontRS** is Rust-native, trait-driven, composable, modular and deterministic. Testing and mocking is a lot easier with MontRS.

## Philosophy

- **Compile-time correctness**: Type-driven design; traits and typed configs everywhere.
- **Leptos Core**: Powered by Leptos for high-performance reactive UI.
- **Modular Architecture**:
    - [Core](packages/core/README.md): Runtime, Module traits, and AppSpec.
    - [Schema](packages/schema/README.md): Validation and Schema macros.
    - [ORM](packages/orm/README.md): SQL-centric ORM with SQLite and Postgres support.
    - [Test](packages/test/README.md): Deterministic testing suite (Unit, Integration, E2E).
    - [Bench](packages/bench/README.md): Professional-grade benchmarking tools.
    - [CLI](packages/cli/README.md) for orchestration.

## Prerequisites

Before installing MontRS, ensure you have the following installed:

- **Rust**: Latest stable version (install via [rustup](https://rustup.rs/)).

- **OpenSSL**: Development headers are required for building dependencies.
    - **Linux**: `sudo apt install libssl-dev pkg-config` (Ubuntu/Debian) or `sudo dnf install openssl-devel` (Fedora).
    - **macOS**: `brew install openssl`.

- **Perl (Windows only)**: Required for building `openssl-sys` (vendored).
    - Install via winget: `winget install StrawberryPerl.StrawberryPerl`

## Getting Started

### Install the CLI

#### Recommended
To install the latest version of the framework including the CLI, use:

```bash
cargo install --locked montrs
```

#### CLI Package
To install only the CLI tool, use:

```bash
cargo install --locked montrs-cli
```

#### Local Path
If you're working on the MontRS repository, you can install the CLI from the local path:

```bash
cargo install --path packages/cli
```

### Create a new app
```bash
montrs new my-app
cd my-app
montrs serve
```

### Benchmarking
Run standard benchmarks or use the native mode for quick file/binary testing:
```bash
# Standard benchmark
montrs bench

# Native mode (no project overhead)
montrs bench --simple ./my-script.rs
montrs bench --simple ./my-binary
```

## 🛠 Project Structure

- `packages/core`: Core meta-framework logic and Leptos integration.
- `packages/schema`: Type-safe validation and schema definitions.
- `packages/orm`: Flexible database backend traits and drivers.
- `packages/test`: TestRuntime, E2E drivers, and unit testing utilities.
- `packages/bench`: Performance benchmarking framework.
- `packages/cli`: The official build and serve tool.
- `templates/`: Project blueprints (including `todo` and `default`).

## License

MontRS is licensed under the [Apache-2.0 license](LICENSE-APACHE) or the [MIT license](LICENSE-MIT), at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in MontRS by you, as defined in the Apache License, shall be dual-licensed as above, without any additional terms or conditions.
