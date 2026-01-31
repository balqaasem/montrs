# Workflow: Fixing Errors

This guide defines the standard procedures for identifying, analyzing, and resolving errors in both MontRS applications and the framework itself.

---

## 📱 For Application Developers

When your application has bugs, compilation errors, or structural violations:

1.  **Identify**: Run `montrs agent list-errors --status Pending` to see the current list of active issues.
2.  **Diagnose**: Use `montrs agent diff <path_to_error_file>` (or the `agent_diff` MCP tool) to get a diagnostic report.
3.  **Analyze**: Follow the LLM instructions provided in the diff report:
    -   Examine the error context.
    -   Locate the root cause in the source code.
4.  **Fix**: Apply the minimal change needed to resolve the issue.
5.  **Validate**: Run `montrs agent check` to ensure no structural invariants were broken.
6.  **Verify**: Run `cargo test` or `montrs test` to ensure functional correctness.
7.  **Clean Up**: Once the error is resolved and verified, the agent will automatically mark it as `Fixed` in the next `montrs spec` run.

---

## 🏗️ For Framework Contributors

When a package in the MontRS workspace (e.g., `core`, `agent`, `cli`) has an error:

1.  **Monitor**: Run `montrs agent list-errors` across the workspace.
2.  **Isolate**: Identify which package owns the error.
3.  **Trace**: Use `montrs agent diff` to see if the error is caused by a violation of framework invariants (e.g., breaking `State Locality`).
4.  **Fix**: Apply the fix to the framework source. Ensure you maintain **Zero-Cost Abstractions** and **Determinism**.
5.  **Audit**: Run `montrs agent doctor --package <package_name>` to verify framework health.
6.  **Full Suite**: Run `cargo test --workspace` to ensure no regressions were introduced in other packages.
7.  **Metadata Update**: Run `montrs spec` to ensure the fix is reflected in the machine-readable `agent.json`.

---

## 🤖 Agent Instructions (MCP)
-   **Step 1**: `agent_list_errors({"status": "Pending"})`
-   **Step 2**: `agent_diff({"path": "..."})`
-   **Step 3**: `get_project_snapshot({})` to find relevant code.
-   **Step 4**: Apply fix.
-   **Step 5**: `agent_check({})`
