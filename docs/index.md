# MontRS Documentation Index

Welcome to the MontRS documentation. This folder contains deep-dive guides for both application developers and framework contributors, with a focus on our AI-First philosophy.

## 🎯 Audience Paths

### For Application Developers
- [First 30 Minutes](getting-started/first-30-minutes.md) - **Start here!** Your first onboarding experience.
- [Introduction](getting-started/introduction.md) - Build your first MontRS app.
- [The Golden Path](getting-started/golden-path.md) - Idiomatic MontRS development.
- [Router](core/router.md) & [Modules](core/modules.md) - Understanding the Loader/Action pattern.
- [Schema & Validation](core/schema.md) - Type-safe data handling.
- [ORM Layer](orm/index.md) - Working with databases.
- [ORM Backends](orm/backends.md) - Supported databases.
- [Testing](testing/index.md) - Writing deterministic tests.
- [Benchmarking](testing/benchmarking.md) - Measuring performance.
- [Deployment](community/deployment.md) - Shipping to production.

### For Framework Contributors
- [Architecture Overview](architecture/overview.md) - The "Shape" of MontRS.
- [AppSpec & Discovery](ai/appspec.md) - How the framework maps itself.
- [Package Boundaries](architecture/packages.md) - Responsibility of each crate.
- [Invariants & Philosophy](architecture/philosophy.md) - The rules we don't break.
- [AI-First Design](ai/ai-first.md) - Principles of machine-readability.
- [CLI Internals](tooling/cli.md) - Orchestration and task runners.
- [Custom Tasks](tooling/tasks.md) - Automating workflows.
- [Contributing Guide](community/contributing.md) - How to build MontRS.

### For AI Agents
- [AI Condensed Onboarding](ai/onboarding.md) - **Recommended for AI agents.**
- [AI Integration Guide](ai/ai-first.md) - How to use MontRS as a tool.
- [Spec Snapshot](ai/spec.md) - Understanding `llm.json`.
- [Error Handling Spec](core/errors.md) - Versioned error files and diffs.
- [Metadata Standards](ai/metadata.md) - How we annotate code for you.

---

## 🏗️ Core Concepts

- **Determinism**: Every part of MontRS is designed to be predictable and testable in isolation.
- **Model-First**: We prioritize machine-readable metadata to enable AI-assisted development.
- **Trait-Driven**: Interfaces are defined by traits, allowing for modular and swappable components.
