---
partial_kind: storage-partial
storage: local-files-json
---

# Local Files JSON

Use `local-files-json` when an adapter persists application data in one or more local JSON files.

## Default Fit

- command-line adapters
- desktop or other local single-user tools
- projects whose stored shape naturally maps to structured JSON documents

## Default Compatibility Rule

Do not combine `local-files-json` with web adapters unless a project explicitly requires local JSON persistence on the server.
