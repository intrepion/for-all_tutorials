<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../../../README.md) / [Setups](../../../../../README.md) / [Code](../../../../README.md) / [dotnet](../../../README.md) / [Adapters](../../README.md) / [Command Line](../README.md) / all.md
<!-- breadcrumbs:end -->

# All

Reusable `.NET` adapter setup for the `command-line/all` target.

## Goal

Create a thin console adapter that can call a well-tested core library without moving project rules into the entry point.

## Use This After

- [Toolchain](../../../toolchain.md)
- one guide from [Testing](../../../testing/README.md)

## Scope

This guide covers:

- scaffolding a console adapter with `dotnet new console`
- adding the adapter to the solution
- referencing the core library from the adapter
- keeping the adapter thin

This guide does not cover:

- project-specific business rules
- test-framework-specific assertions or templates
- project-specific command parsing

## Suggested Scaffold Commands

Replace the placeholders with the names used by the project tutorial:

```bash
dotnet new console --name <adapter-name> --output <solution-root>/src/<adapter-name> --framework <target-framework>
dotnet sln <solution-root>/<solution-name>.sln add <solution-root>/src/<adapter-name>/<adapter-name>.csproj
dotnet add <solution-root>/src/<adapter-name>/<adapter-name>.csproj reference <solution-root>/src/<library-name>/<library-name>.csproj
```

## Suggested File Shape

```text
<solution-root>/
  <solution-name>.sln
  src/
    <library-name>/
      <library-name>.csproj
    <adapter-name>/
      <adapter-name>.csproj
      Program.cs
```

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
- the console project references the core library
- the console entry point can delegate to the core library
