---
partial_kind: storage-partial
storage: database-sqlite
---

# Database SQLite

Use `database-sqlite` when an adapter persists data in SQLite.

## Default Fit

- local apps that still benefit from a relational database
- small web apps or desktop apps with one-process persistence
- projects that need database semantics without a separate database server

## Default Compatibility Rule

This is the lightest database-family slot for adapters that need persistence but do not need a separate hosted database service.
