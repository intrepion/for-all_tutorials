---
partial_kind: storage-partial
storage: local-files-toml
---

# Local Files TOML

Use `local-files-toml` when an adapter persists application data in one or more local TOML files.

## Default Fit

- command-line adapters
- desktop or other local single-user tools
- projects that benefit from a structured text format commonly used for local configuration-style data

## Default Compatibility Rule

Do not combine `local-files-toml` with web adapters unless a project explicitly requires local TOML persistence on the server.
