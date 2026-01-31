# MCP Setup & Access Guide

The MontRS MCP (Model Context Protocol) server enables deep integration between your AI agents and the MontRS framework. This guide covers how to set up the server and grant agents access.

## 🚀 Setting Up the Server

The MCP server is built into the `montrs` CLI.

### 1. Ensure CLI is Installed
```bash
cargo install --path packages/cli --bin montrs --force
```

### 2. Start the Server
The server communicates via standard input/output (stdio), which is the standard for MCP.
```bash
montrs mcp serve
```

## 🔗 Connecting Agents

Depending on your agent environment, follow the steps below to connect.

### For Trae / Cursor / Antigravity
To let the built-in IDE agent access MontRS tools:

1.  Open your IDE settings.
2.  Navigate to **MCP Servers** or **Tools**.
3.  Add a new **Stdio** server:
    -   **Name**: `MontRS`
    -   **Command**: `montrs`
    -   **Arguments**: `["mcp", "serve"]`

### For Custom Agents (Python/Node)
If you are building your own agent, use an MCP client library to spawn the `montrs mcp serve` process.

**Example (Conceptual):**
```python
from mcp import Client, StdioClientTransport

async with Client(StdioClientTransport("montrs", ["mcp", "serve"])) as client:
    tools = await client.list_tools()
    print(f"Available tools: {tools}")
```

## 🛠️ Available Tools

Once connected, your agent will have access to:

| Tool | Description |
| :--- | :--- |
| `agent_check` | Validates project structure and invariants. |
| `agent_doctor` | Checks framework health and agent-readability. |
| `agent_diff` | Analyzes errors and provides fix instructions. |
| `get_project_snapshot` | Returns full machine-readable project metadata. |
| `agent_list_errors` | Returns structured list of active/resolved issues. |

## 🔒 Security & Permissions

The MCP server runs with the same permissions as the `montrs` CLI. It can:
-   Read project files.
-   Write to the `.agent` directory.
-   Execute internal framework validation.

It **cannot** access files outside the current project root unless explicitly directed via path arguments.

---
*For more details on how to use these tools effectively, see the [Agentic CLI & MCP Workflows](agentic-workflows.md) guide.*
