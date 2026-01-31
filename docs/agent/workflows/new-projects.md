# Workflow: New Projects & Packages

This guide defines how to initialize new MontRS applications and how to add new framework packages to the workspace.

---

## 📱 For Application Developers (New Project)

1.  **Initialize**: Run `montrs new <project_name> --template <template_name>`.
2.  **Onboard**: Run `montrs spec` immediately to generate the initial `.agent/agent.json`.
3.  **Explore**: Read the generated `agent.json` or `agent.txt` to understand the scaffolded structure.
4.  **Configure**: Update `montrs.toml` with project-specific metadata.
5.  **First Plate**: Use `montrs generate plate <name>` to create your first domain boundary.
6.  **Verify**: Run `montrs agent check` to ensure the scaffold is valid.

---

## 🏗️ For Framework Contributors (New Package)

1.  **Scaffold**: Create a new directory in `packages/`.
2.  **Initialize**: Run `cargo init --lib`.
3.  **Workspace Link**: Add the new package to the top-level `Cargo.toml` workspace members.
4.  **Agent-Ready**: 
    -   Create a `README.md` with mandatory sections (What it is, What it solves, etc.).
    -   Add `montrs-agent` as a dev-dependency if metadata collection is needed.
5.  **Metadata**: Annotate public tools with `@agent-tool`.
6.  **Register**: Run `montrs spec` to ensure the new package appears in the project snapshot.
7.  **Doctor**: Run `montrs agent doctor --package <new_package>` to verify it meets framework standards.

---

## 🤖 Agent Instructions (MCP)
-   **Step 1**: Use CLI `montrs new` for apps.
-   **Step 2**: `get_project_snapshot({})` to verify the new structure.
-   **Step 3**: `agent_doctor({"package": "..."})` for framework contributions.
