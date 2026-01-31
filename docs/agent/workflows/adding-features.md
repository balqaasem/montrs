# Workflow: Adding New Features

This guide defines the "Golden Path" for adding functionality to an app or extending the framework.

---

## 📱 For Application Developers

1.  **Define Schema**: Create input/output structs with `#[derive(Schema)]`.
2.  **Implement Logic**: 
    -   Create a `Loader` for read-only data.
    -   Create an `Action` for state changes.
3.  **Encapsulate**: Add the logic to a `Plate`.
4.  **Expose**: Register the route in `Plate::register_routes`.
5.  **Annotate**: Implement `description()` and `input_schema()` for agent discovery.
6.  **Verify**: Run `montrs spec` and `montrs agent check`.

---

## 🏗️ For Framework Contributors

1.  **Trait Definition**: Define new core traits in `packages/core`.
2.  **Implementation**: Provide default or specialized implementations in relevant packages.
3.  **Macro Support**: If the feature requires automation, update `packages/schema` (procedural macros).
4.  **CLI Integration**: Add new commands to `packages/cli` if necessary.
5.  **Agent Visibility**: 
    -   Update `packages/agent` to collect metadata for the new feature.
    -   Add `@agent-tool` to the new implementation.
6.  **Template Update**: Add an example of the feature to `templates/todo`.
7.  **Final Audit**: Run `montrs spec` and `agent doctor`.

---

## 🤖 Agent Instructions (MCP)
-   **Step 1**: Follow the **Golden Path** (Schema -> Logic -> Plate -> Route).
-   **Step 2**: Always add `description()` to new traits.
-   **Step 3**: `agent_check({})` to verify compliance.
