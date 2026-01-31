# 🤖 MontRS Agent Entry Point

Welcome to the unified command center for MontRS Agents. This document is the "Map of Maps"—it defines the operational framework for how you should interact with the codebase based on your current intent.

---

## 🧭 Operational Framework: "What are you doing?"

Choose your path based on the task at hand. Do not attempt to "wing it"—follow the established workflows to maintain architectural integrity.

### 1. I am fixing a Bug or Error
- **Goal**: Identify, diagnose, and resolve a failure (compilation, runtime, or architectural).
- **Primary Guide**: [Workflow: Fixing Errors](workflows/fixing-errors.md)
- **Key Tools**: `montrs agent list-errors`, `montrs agent diff`.

### 2. I am adding a New Feature
- **Goal**: Extend the application or framework with new capabilities.
- **Primary Guide**: [Workflow: Adding Features](workflows/adding-features.md)
- **Key Tools**: `montrs generate plate`, `montrs generate route`, `montrs spec`.

### 3. I am starting a New Project
- **Goal**: Scaffold a fresh MontRS application from scratch.
- **Primary Guide**: [Workflow: New Projects](workflows/new-projects.md)
- **Key Tools**: `montrs new`.

### 4. I am restructuring or Refactoring
- **Goal**: Move logic, split plates, or improve architectural health without changing behavior.
- **Primary Guide**: [Workflow: Restructuring](workflows/restructuring.md)
- **Key Tools**: `montrs agent check`.

---

## 🎭 System Prompts (Your Identity)

Your behavior is governed by your specific role. Read these to understand your constraints and core identity:

- **[App Developer Prompt](app-developer-prompt.md)**: For building applications *using* MontRS.
- **[Framework Contributor Prompt](framework-contributor-prompt.md)**: For developing and maintaining the *framework itself*.

---

## 🛡️ Foundational Knowledge

Before performing any action, ensure you are grounded in these core principles:

- **[Agent-First Philosophy](agent-first.md)**: Why we prioritize machine-readability.
- **[Framework Invariants](onboarding.md)**: The rules you must never break.
- **[Spec Snapshot (agent.json)](spec.md)**: How to read the project's current state.
- **[Metadata Standards](metadata.md)**: How to annotate code for discovery.

---

## 🔌 Tooling & Integration

- **[MCP Setup & Access](mcp-setup.md)**: How to connect your capabilities to the project.
- **[Agentic CLI Guide](agentic-workflows.md)**: Mastering the command-line loop.
