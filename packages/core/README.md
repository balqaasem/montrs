# montrs-core

The core architectural engine for MontRS. This package provides the foundational traits and structs that define how a MontRS application is structured, initialized, and executed.

## Overview

`montrs-core` defines the "Shape" of a MontRS application. It provides the abstractions for Modules, Routing (Loaders/Actions), Validation, Environment Management, and Feature Flags. It is designed to be highly modular and portable across different targets (Server, WASM, Edge, etc.).

## Key Components

- **Module Trait**: The unit of composition. Every logical feature in a MontRS app is a module.
- **Router (Loaders & Actions)**: The unified routing system. Loaders handle data fetching, and Actions handle state mutations.
- **AppConfig Trait**: Defines the global configuration and error types for the application.
- **AiError Trait**: A foundational trait for providing structured, machine-readable metadata for errors.
- **Validate Trait**: A schema validation system with AI-discoverable rules.

## AI-First Features

`montrs-core` is the source of truth for AI-readable metadata:
- **Trait Metadata**: `Module`, `Loader`, and `Action` all provide `description()` and `metadata()` methods.
- **Error Metadata**: The `AiError` trait ensures every error in the system can be explained to an AI model with codes, context, and remediation hints.
- **Schema Discovery**: `Validate` and `Router` components expose their schemas (input/output) for AI-assisted code generation.

## Integration

All other MontRS packages depend on `montrs-core` for basic types and traits. When building a MontRS app, you primarily interact with the traits defined here.

## Metadata

- **Subsystem**: `core`
- **AI-Native**: Yes
- **Version**: 0.1.0
