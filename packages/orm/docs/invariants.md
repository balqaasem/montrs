# ORM Package Invariants

## 1. Responsibility
`montrs-orm` provides a type-safe, agent-readable interface for database operations and migrations.

## 2. Invariants
- **Backend Agnostic**: Core traits must remain independent of specific database backends (SQL, NoSQL, etc.).
- **Type-Safe Queries**: All queries should be validated at compile-time or through type-safe DSLs defined in this package.
- **Deterministic Migrations**: Migration logic must be reversible and idempotent.

## 3. Boundary Definitions
- **In-Scope**: DB traits, migration orchestration, type-safe query builders.
- **Out-of-Scope**: Direct network handling (delegated to drivers), application business logic.
