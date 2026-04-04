---
partial_kind: storage-partial
ecosystem: dotnet
surface: command-line
storage: local-files-json
---

# Local Files JSON

Use `local-files-json` for `.NET` command-line adapters that persist application data in local JSON files.

## Use This After

- [Shared Storage Taxonomy](../../../../storage/local-files-json.md)
- [All](../../adapters/command-line/all.md)

## Guidance

- keep file-path selection, directory creation, and missing-file handling in the adapter
- use `System.IO` to read and write the full JSON document
- prefer `System.Text.Json` for adapter-owned JSON serialization unless the chosen project explicitly needs something else
- keep canonical parsing, validation, and project behavior in the core when the spec defines those contracts there
- keep the adapter thin: read the file, call the core, write the file

## Ready State

The `.NET` `local-files-json` command-line storage path is ready when:

- the adapter can create or locate the JSON file it owns
- the adapter can read the current JSON document from disk
- the adapter can write the updated JSON document back to disk
- JSON storage mechanics do not swallow project rules that belong in the core
