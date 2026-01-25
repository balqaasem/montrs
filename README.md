<!-- MontRS: The Rust-native, trait-driven deterministic web framework. -->

# MontRS

**MontRS** is a Rust-native, trait-driven, deterministic meta-framework built on **Leptos 0.8**. It blends the engineering strengths of Leptos (fine-grained reactivity), Remix (web-native routing), and Drizzle (minimal ORM).

## 🚀 Philosophy

- **Compile-time correctness**: Type-driven design; traits and typed configs everywhere.
- **Leptos Core**: Powered by Leptos for high-performance reactive UI.
- **Modular Architecture**:
    - [Core](packages/core/README.md): Runtime, Module traits, and AppSpec.
    - [Schema](packages/schema/README.md): Validation and Schema macros.
    - [ORM](packages/orm/README.md): SQL-centric ORM with SQLite and Postgres support.
    - [Test](packages/test/README.md): Deterministic testing suite.
- **Production CLI**: [cargo-mont](packages/cargo-mont/README.md) for orchestration.

## 📋 Prerequisites

Before installing MontRS, ensure you have the following installed:

- **Rust**: Latest stable version (install via [rustup](https://rustup.rs/)).

- **OpenSSL**: Development headers are required for building dependencies.
    - **Linux**: `sudo apt install libssl-dev pkg-config` (Ubuntu/Debian) or `sudo dnf install openssl-devel` (Fedora).
    - **macOS**: `brew install openssl`.

- **Perl (Windows only)**: Required for building `openssl-sys` (vendored).
    - Install via winget: `winget install StrawberryPerl.StrawberryPerl`

## 📦 Getting Started

### Install the CLI

#### Published Version
To install the latest published version of `cargo-mont`, use:

```bash
cargo install --locked cargo-mont
```

#### Local Path
If you're working on the MontRS repository, you can install the CLI from the local path:

```bash
cargo install --path packages/cargo-mont
```

### Create a new app
```bash
cargo mont new my-app
cd my-app
cargo mont serve
```

## 🛠 Project Structure

- `packages/core`: Core meta-framework logic and Leptos integration.
- `packages/schema`: Type-safe validation and schema definitions.
- `packages/orm`: Flexible database backend traits and drivers.
- `packages/test`: TestRuntime for unit and integration testing.
- `packages/cargo-mont`: The official build and serve tool.
- `templates/`: Project blueprints (including `todo` and `default`).

## 📄 License

This project is licensed under the MIT License.
