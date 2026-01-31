# Workflow: Restructuring Projects

This guide defines how to safely reorganize code, move routes, and refactor package boundaries in a MontRS environment.

---

## 📱 For Application Developers

1.  **Plan**: Identify which routes or services belong in a different `Plate`.
2.  **Move Logic**: Relocate `Loader` or `Action` implementations to the target Plate.
3.  **Update Routing**: 
    -   Remove route registration from the old Plate.
    -   Add route registration to the new Plate.
4.  **Sync**: Run `montrs spec` to update the architectural snapshot.
5.  **Validate**: Run `montrs agent check`. This will detect if any dependencies are now broken or if routes are missing.
6.  **Cleanup**: Remove any unused imports or services from the old Plate.

---

## 🏗️ For Framework Contributors

1.  **Refactor Boundaries**: When moving logic between packages (e.g., from `core` to `utils`):
    -   Update `Cargo.toml` dependencies for all affected packages.
    -   Ensure public APIs remain stable or provide a migration path.
2.  **Maintain Annotations**: Ensure `@agent-tool` markers are moved along with the code.
3.  **Global Check**: Run `montrs agent doctor` for the entire workspace.
4.  **Verification**: 
    -   Run `cargo test --workspace`.
    -   Run `montrs spec` and compare the new `agent.json` with a previous version to ensure no unintended metadata loss.

---

## 🤖 Agent Instructions (MCP)
-   **Step 1**: `get_project_snapshot({})` to map current dependencies.
-   **Step 2**: Apply structural changes.
-   **Step 3**: `agent_check({})` to find broken invariants.
-   **Step 4**: `agent_doctor({})` for workspace-wide health.
