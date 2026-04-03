---
partial_kind: storage-root
---

# Storage

Storage and database setup partials for compiled tutorials.

The storage slot in every adapter repo name and manifest should use one of these canonical values:

- [No Storage](no-storage.md)
- [Local Files CSV](local-files-csv.md)
- [Local Files JSON](local-files-json.md)
- [Local Files YAML](local-files-yaml.md)
- [Local Files TOML](local-files-toml.md)
- [Local Files XML](local-files-xml.md)
- [Database Firebase](database-firebase.md)
- [Database SQLite](database-sqlite.md)
- [Database Postgres](database-postgres.md)
- [Database MySQL](database-mysql.md)

## Default Compatibility Rule

- `local-files-*` slots are intended for command-line, desktop, and other local single-user adapters
- `local-files-*` should not be combined with web adapters unless a project explicitly requires that shape
- `database-*` slots are the default family to consider for web adapters or multi-user deployments
- use `no-storage` when a run does not persist data and does not call a storage service
