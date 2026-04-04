<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../../../README.md) / [Setups](../../../../../README.md) / [Code](../../../../README.md) / [dotnet](../../../README.md) / [Adapters](../../README.md) / [Command Line](../README.md) / all.md
<!-- breadcrumbs:end -->

# All

Reusable `.NET` adapter setup for the `command-line/all` target.

## Goal

Create a thin console adapter that can call a well-tested core library without moving project rules into the entry point.

## Use This After

- [Toolchain](../../../toolchain/README.md)
- one guide from [Testing](../../../testing/README.md)
- an existing core library repo for the project logic

## Scope

This guide covers:

- scaffolding a console adapter repo with `dotnet new console`
- adding the adapter project to its solution
- adding a dependency on the core library from the adapter
- keeping the adapter thin
- pairing the adapter with a stack-specific storage guide when the project persists local data

This guide does not cover:

- project-specific business rules
- test-framework-specific assertions or templates
- project-specific command parsing
- local-file storage mechanics beyond choosing the appropriate storage guide
- command-line framework choices such as [Spectre Console](../../../frameworks/command-line/spectre-console.md)

## Suggested Scaffold Commands

Replace the placeholders with the names used by the project tutorial:

```bash
dotnet new sln --format sln --name <solution-name> --output <solution-root>
dotnet new console --name <adapter-name> --output <solution-root>/src/<adapter-name> --framework <target-framework>
dotnet sln <solution-root>/<solution-name>.sln add <solution-root>/src/<adapter-name>/<adapter-name>.csproj
dotnet add <solution-root>/src/<adapter-name>/<adapter-name>.csproj package <core-package-name> --version <core-package-version>
```

## Suggested File Shape

```text
<solution-root>/
  <solution-name>.sln
  src/
    <adapter-name>/
      <adapter-name>.csproj
      Program.cs
```

The core library repo should stay separate. The adapter repo should depend on it as a real dependency rather than by copying the core source files.

If the chosen project persists local data, pair this guide with one storage guide from [Storage](../../storage/command-line/README.md).

## Adapter Rule

The `Program.cs` entry point should stay thin:

- read input from the command line
- call the core library
- write the output

Do not duplicate validation or service rules in the adapter if those rules belong in the core project logic.

## Ready State

The `.NET` `command-line/all` adapter setup is ready when:

- the console project exists
- the console project is added to the solution
- the console project depends on the core library
- the console entry point can delegate to the core library
- any optional command-line framework still delegates to the core library
