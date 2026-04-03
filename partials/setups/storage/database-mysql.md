---
partial_kind: storage-partial
storage: database-mysql
---

# Database MySQL

Use `database-mysql` when an adapter persists data in MySQL.

## Default Fit

- web adapters
- multi-user deployments
- projects that target MySQL-compatible relational storage

## Default Compatibility Rule

Prefer this slot over `local-files-*` for hosted web applications unless the project explicitly calls for a different storage shape.
