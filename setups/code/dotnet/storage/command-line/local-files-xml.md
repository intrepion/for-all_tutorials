<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../../../README.md) / [Setups](../../../../../README.md) / [Code](../../../../README.md) / [dotnet](../../../README.md) / [Storage](../../README.md) / [Command Line](README.md) / Local Files XML
<!-- breadcrumbs:end -->

# Local Files XML

Use `local-files-xml` for `.NET` command-line adapters that persist application data in local XML files.

## Use This After

- [Shared Storage Taxonomy](../../../../storage/local-files-xml.md)
- [All](../../adapters/command-line/all.md)

## Guidance

- keep file-path selection, directory creation, and missing-file handling in the adapter
- use `System.IO` to read and write the full XML document as UTF-8 text
- use `.NET` XML APIs or serializer support in the adapter for the chosen XML shape
- keep canonical parsing, validation, and project behavior in the core when the spec defines those contracts there
- keep the adapter thin: read the file, call the core, write the file

## Ready State

The `.NET` `local-files-xml` command-line storage path is ready when:

- the adapter can create or locate the XML file it owns
- the adapter can read the current XML document from disk
- the adapter can write the updated XML document back to disk
- XML mapping details stay contained to the adapter layer
