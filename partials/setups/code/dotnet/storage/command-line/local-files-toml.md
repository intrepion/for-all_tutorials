---
partial_kind: storage-partial
ecosystem: dotnet
surface: command-line
storage: local-files-toml
---

# Local Files TOML

Use `local-files-toml` for `.NET` command-line adapters that persist application data in local TOML files.

## Use This After

- [Shared Storage Taxonomy](../../../../storage/local-files-toml.md)
- [All](../../adapters/command-line/all.md)

## Guidance

- keep file-path selection, directory creation, and missing-file handling in the adapter
- use `System.IO` to read and write the full TOML document as UTF-8 text
- choose one focused TOML library for the adapter and keep that dependency out of the core library
- keep canonical parsing, validation, and project behavior in the core when the spec defines those contracts there
- keep the adapter thin: read the file, call the core, write the file

## Ready State

The `.NET` `local-files-toml` command-line storage path is ready when:

- the adapter can create or locate the TOML file it owns
- the adapter can read the current TOML document from disk
- the adapter can write the updated TOML document back to disk
- TOML library choices stay contained to the adapter layer
