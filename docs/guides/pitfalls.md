# Common Pitfalls and Non-Patterns

While MontRS is designed to be flexible, certain patterns can lead to complexity, flakiness, or loss of agent-readiness. Avoid these common pitfalls.

## 🚫 Mixing Business Logic in Main

**Don't**: Put all your route registrations and logic in `main.rs`.
**Do**: Use `Plate` implementations to keep your application organized and composable.

## 🚫 Skipping Schema Validation

**Don't**: Use raw `Value` or unvalidated structs for API inputs.
**Do**: Always use `#[derive(Schema)]` to ensure data integrity and provide metadata to agents.

## 🚫 Non-Deterministic Tests

**Don't**: Rely on global state, system time, or external APIs in your tests without mocking.
**Do**: Use `montrs-test` fixtures and the `TestRuntime` to ensure your tests are deterministic and repeatable.

## 🚫 Ignoring Agent Metadata

**Don't**: Leave `description()` as the default `None` for your core traits.
**Do**: Provide brief, clear descriptions. This is the primary way agents understand the "Intent" of your code.

## 🚫 Manual Snapshots

**Don't**: Try to manually edit `.agent/agent.json`.
**Do**: Always use `montrs spec` to regenerate the specification from your source code.

## 🚫 Direct Database Access in Loaders

**Don't**: Hardcode database connections inside every loader.
**Do**: Use the `ctx.db()` provided by the framework to ensure proper connection pooling and backend abstraction.

## 🚫 Complex Macros for Logic

**Don't**: Hide significant business logic inside custom procedural macros.
**Do**: Use traits and functions. Agents and human reviewers find explicit code much easier to reason about.
