# Workflow: Fixing Errors

This guide defines the standard procedures for identifying, analyzing, and resolving errors in both MontRS applications and the framework itself.

---

## 📱 For Application Developers

When your application has bugs, compilation errors, or structural violations:

1.  **Identify**: Run `montrs agent list-errors --status Pending`.
2.  **Contextualize**: 
    - Use `montrs agent diff <path_to_error_file>` (or the `agent_diff` MCP tool) to get a diagnostic report.
    - **Read Framework Invariants**: Consult the `docs/invariants.md` for the framework packages you are utilizing to see if you have violated any usage rules.
3.  **Analyze**: Use the diagnostic report to understand the root causes of the specific errors or bugs identified in Step 1.
    -   Examine the error context.
    -   Locate the root cause in the source code.
4.  **Fix**: Apply the minimal change needed to resolve the issue.
5.  **Validate**: Run `montrs agent check` to ensure no structural invariants were broken.
6.  **Verify**: Run `cargo test` or `montrs test` to ensure functional correctness.
7.  **Clean Up**: Once the error is resolved and verified, the agent will automatically mark it as `Fixed` in the next `montrs spec` run.

---

## 🏗️ For Framework Contributors

When a framework package (e.g., `core`, `agent`, `cli`) has an error:

1.  **Monitor**: Run `montrs agent list-errors` across the workspace.
2.  **Isolate & Contextualize**: 
    - Identify which framework package owns the error.
    - Read its `docs/invariants.md` to understand its internal architectural constraints.
3.  **Trace**: Use `montrs agent diff` to see if the error is caused by a violation of framework-wide or package-specific internal invariants.
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
