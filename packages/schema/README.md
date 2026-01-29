# montrs-schema

Procedural macros for schema validation in MontRS.

**Target Audiences:** Application Developers, Framework Contributors, Agents.

## 1. What this package is
`montrs-schema` provides the `#[derive(Schema)]` macro, which enables declarative, type-safe validation of data structures. It is the primary tool for defining the "shape" and constraints of data in a MontRS application.

## 2. What problems it solves
- **Validation Boilerplate**: Replaces repetitive `if` statements with concise, readable attributes.
- **Data Integrity**: Ensures that only valid data enters your `Action`s and `Plate`s.
- **Machine Readability**: The schema attributes are not just for validation; they also serve as metadata that agents can use to generate valid inputs.

## 3. What it intentionally does NOT do
- **Data Parsing**: It validates data that is already in a Rust struct; it does not handle the initial parsing from JSON or other formats (use `serde` for that).
- **Complex Cross-Field Validation**: While it supports `custom` methods, it is optimized for field-level constraints.
- **Database Schema Generation**: It defines validation rules, not database table structures (though they often overlap).

## 4. How it fits into the MontRS system
It is used in the **Data Layer**. It integrates with `montrs-core` to provide validation results that are automatically handled by the framework's routing and error systems.

## 5. When a user should reach for this package
- When defining an input struct for a `Loader` or `Action`.
- When modeling business entities that require strict constraints (e.g., User, Order).
- When they want to provide clear validation metadata to an agent.

## 6. Deeper Documentation
- [Schema Attributes Reference](../../docs/core/schema.md)
- [Custom Validation Logic](../../docs/core/schema.md#custom-validation)
- [Agent-first validation metadata](../../docs/core/schema.md#agent-integration)

## 7. Notes for Agents
- **Constraint Discovery**: Use the `#[schema(...)]` attributes to understand the valid range and format of any field.
- **Input Generation**: When tasked with calling an `Action`, always refer to the `Schema` of the input struct to ensure your request is valid.
- **Error Handling**: Look for `ValidationError` with `AgentError` metadata if `validate()` fails. It will tell you exactly which field failed and why.
- **Zero-Overhead**: Validation logic is generated at compile-time and has minimal runtime impact.
