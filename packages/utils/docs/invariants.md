# Utils Package Invariants

## 1. Responsibility
`montrs-utils` contains generic, reusable utilities that are used across multiple packages in the workspace.

## 2. Invariants
- **Side-Effect Free**: Functions in this package should ideally be pure and free of side effects.
- **Generic Utility**: Only include logic that is truly generic and not specific to a single framework component.
- **High Stability**: As a low-level dependency, changes here must be carefully vetted for breaking impact.

## 3. Boundary Definitions
- **In-Scope**: String manipulation, collection helpers, common algorithm implementations.
- **Out-of-Scope**: Framework-specific logic (Plates, Routes, etc.).
