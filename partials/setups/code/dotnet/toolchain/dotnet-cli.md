---
partial_kind: toolchain-item
ecosystem: dotnet
tool: dotnet-cli
---

# dotnet CLI

Primary command-line workflow for creating solutions, libraries, apps, and test projects.

Use this guide for scaffold commands and repeatable repo conventions.

## Stable Solution Format

Use `dotnet new sln --format sln` for tutorial runs.

This keeps the solution filename predictable as `<solution-name>.sln` instead of relying on the newer default `.slnx` format.

## Core Scaffold Pattern

For a core library repo, run:

```bash
dotnet new sln --format sln --name <solution-name>
dotnet new gitignore
dotnet new classlib --name <library-project-name> --output src/<library-project-name>
```

If the tutorial does not explicitly pin a target framework, use the SDK default. If you do pin a framework, use the same value consistently for the library and its test project.
