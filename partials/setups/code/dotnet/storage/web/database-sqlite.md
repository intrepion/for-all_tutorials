---
partial_kind: storage-partial
ecosystem: dotnet
surface: web
storage: database-sqlite
---

# Database SQLite

Use `database-sqlite` for `.NET` `web/full-stack` adapters that need a lightweight persisted relational database.

## Use This After

- [Shared Storage Taxonomy](../../../../storage/database-sqlite.md)
- [Full Stack](../../adapters/web/full-stack.md)

## Guidance

- keep the SQLite connection string, `DbContext`, migrations, and adapter-owned persistence code in the web adapter repo
- prefer `Microsoft.EntityFrameworkCore.Sqlite` when the adapter needs a relational persistence layer with minimal infrastructure
- map database records to and from the core library contract at the adapter boundary
- keep canonical validation, formatting, and domain rules in the core instead of in EF Core entities or controllers
- treat SQLite as a persisted implementation detail of the adapter, not a dependency of the core library

## Ready State

The `.NET` `database-sqlite` web storage path is ready when:

- the web adapter can create or migrate the SQLite schema it owns
- the web adapter can read persisted data from SQLite and write updates back
- SQLite-specific code remains outside the core library
- the chosen web framework still delegates project rules to the core library
