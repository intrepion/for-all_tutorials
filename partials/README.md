---
partial_kind: partials-root
pilot: true
---

# Partials

Source material for future compiled tutorials.

Project partials now cover the full curriculum project set.

The current supported generated tutorial family is:

- core tutorials on `.NET/C# xUnit`
- adapter tutorials on `.NET/C# xUnit` for:
  - `no-storage/command-line/all/no-framework`
  - `no-storage/client/all/no-framework`
  - `no-storage/graphical/all/no-framework`
  - `no-storage/web-service/all/no-framework`
  - `local-files-json/command-line/all/no-framework`
  - `database-firebase/command-line/all/no-framework`
  - `database-sqlite/web/full-stack/blazor-server`

Compiled tutorial outputs should be generated from the project manifests in this tree, not authored by hand.

Every project manifest now declares at least one active compiled tutorial output on the current `.NET/C# xUnit` core path.

Most projects now also declare active `.NET/C# xUnit` adapter outputs on one of those supported storage, surface, target, and framework paths.

## Active Generated Paths

Current generated tutorial paths that are real in this repo:

- `tutorials/<project>/dotnet/csharp/xunit/core/README.md` for 57 projects
- `tutorials/<project>/dotnet/csharp/xunit/adapters/no-storage/command-line/all/no-framework/README.md` for 50 projects
- `tutorials/<project>/dotnet/csharp/xunit/adapters/no-storage/client/all/no-framework/README.md` for 1 project
- `tutorials/<project>/dotnet/csharp/xunit/adapters/no-storage/graphical/all/no-framework/README.md` for 1 project
- `tutorials/<project>/dotnet/csharp/xunit/adapters/no-storage/web-service/all/no-framework/README.md` for 1 project
- `tutorials/<project>/dotnet/csharp/xunit/adapters/local-files-json/command-line/all/no-framework/README.md` for 1 project
- `tutorials/<project>/dotnet/csharp/xunit/adapters/database-firebase/command-line/all/no-framework/README.md` for 1 project
- `tutorials/<project>/dotnet/csharp/xunit/adapters/database-sqlite/web/full-stack/blazor-server/README.md` for 2 projects

The one project that still needs a new storage-slot decision before honest adapter activation is:

- `todo-list`, which currently models persisted task data as plain newline-delimited text rather than one of the existing canonical storage slots

## Contents

- [Projects](projects/README.md)
- [Setups](setups/README.md)
