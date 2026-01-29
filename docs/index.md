# MontRS Documentation Index

Welcome to the MontRS documentation. This folder contains deep-dive guides for both application developers and framework contributors, with a focus on our AI-First philosophy.

## 🎯 Audience Paths

### For Application Developers
- [First 30 Minutes](first-30-minutes.md) - **Start here!** Your first onboarding experience.
- [Getting Started](getting-started.md) - Build your first MontRS app.
- [The Golden Path](golden-path.md) - Idiomatic MontRS development.
- [Router & Modules](router.md) - Understanding the Loader/Action pattern.
- [Schema & Validation](schema.md) - Type-safe data handling.
- [ORM Layer](orm.md) - Working with databases.
- [ORM Backends](orm-backends.md) - Supported databases.
- [Testing](testing.md) - Writing deterministic tests.
- [Benchmarking](benchmarking.md) - Measuring performance.
- [Deployment](deployment.md) - Shipping to production.

### For Framework Contributors
- [Architecture Overview](architecture.md) - The "Shape" of MontRS.
- [AppSpec & Discovery](appspec.md) - How the framework maps itself.
- [Package Boundaries](packages.md) - Responsibility of each crate.
- [Invariants & Philosophy](philosophy.md) - The rules we don't break.
- [AI-First Design](ai-first.md) - Principles of machine-readability.
- [CLI Internals](cli.md) - Orchestration and task runners.
- [Custom Tasks](tasks.md) - Automating workflows.
- [Contributing Guide](contributing.md) - How to build MontRS.

### For AI Agents
- [AI Condensed Onboarding](ai-onboarding.md) - **Recommended for AI agents.**
- [AI Integration Guide](ai-first.md) - How to use MontRS as a tool.
- [Spec Snapshot](spec.md) - Understanding `llm.json`.
- [Error Handling Spec](errors.md) - Versioned error files and diffs.
- [Metadata Standards](metadata.md) - How we annotate code for you.

---

## 🏗️ Core Concepts

- **Determinism**: Every part of MontRS is designed to be predictable and testable in isolation.
- **Model-First**: We prioritize machine-readable metadata to enable AI-assisted development.
- **Trait-Driven**: Interfaces are defined by traits, allowing for modular and swappable components.
