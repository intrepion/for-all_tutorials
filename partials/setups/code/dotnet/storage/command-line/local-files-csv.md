---
partial_kind: storage-partial
ecosystem: dotnet
surface: command-line
storage: local-files-csv
---

# Local Files CSV

Use `local-files-csv` for `.NET` command-line adapters that persist application data in local CSV files.

## Use This After

- [Shared Storage Taxonomy](../../../../storage/local-files-csv.md)
- [All](../../adapters/command-line/all.md)

## Guidance

- keep file-path selection, directory creation, and missing-file handling in the adapter
- use `System.IO` APIs to read and write UTF-8 text files
- keep CSV-specific quoting, escaping, and delimiter concerns at the adapter boundary
- let the core own parsing, validation, and formatting rules when the project spec defines those as core contracts
- keep the adapter thin: read the file, call the core, write the file

## Ready State

The `.NET` `local-files-csv` command-line storage path is ready when:

- the adapter can create or locate the CSV file it owns
- the adapter can read the current CSV text from disk
- the adapter can write updated CSV text back to disk
- project rules still live in the core library instead of in the file I/O code
