---
partial_kind: storage-partial
storage: database-postgres
---

# Database Postgres

Use `database-postgres` when an adapter persists data in PostgreSQL.

## Default Fit

- web adapters
- multi-user deployments
- projects that benefit from a full relational database server

## Default Compatibility Rule

Prefer this slot over `local-files-*` for hosted web applications unless the project explicitly calls for a different storage shape.
