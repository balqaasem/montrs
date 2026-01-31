# MontRS Wrapper Package Invariants

## 1. Responsibility
The `montrs` package is a high-level wrapper that provides the primary entry point for users consuming the framework as a single dependency.

## 2. Invariants
- **Clean Re-exports**: This package must cleanly re-export the public API of core packages (`core`, `orm`, `agent`) to provide a unified experience.
- **Minimal Logic**: This package should contain minimal unique logic, acting primarily as a facade.

## 3. Boundary Definitions
- **In-Scope**: Re-exports, high-level convenience macros, meta-documentation.
- **Out-of-Scope**: Implementation of core framework features.
