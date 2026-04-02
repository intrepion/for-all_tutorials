<!-- breadcrumbs:start -->
[for-all_tutorials](../../../README.md) / [Setups](../../README.md) / [Code](../README.md) / [dotnet](README.md) / toolchain.md
<!-- breadcrumbs:end -->

# Toolchain

Core `.NET` toolchain guidance for project tutorials that use the `dotnet` ecosystem.

## Goal

Set up a clean, repeatable foundation for `.NET` output repos without tying the instructions to any one local machine.

## Scope

This guide covers:

- the base `dotnet` CLI workflow
- solution scaffolding
- class-library scaffolding for core-library repos
- common repository conventions for `.NET` tutorials

This guide does not cover:

- adapter-specific scaffolding
- test-framework-specific scaffolding
- framework-specific bootstrapping
- project-specific business logic

## Recommended Conventions

- use SDK-style projects
- keep source under `src/`
- keep tests under `tests/`
- keep nullable reference types enabled when the chosen language supports them
- keep a small solution or equivalent workspace root when the stack supports it

## Verify The Toolchain

Before starting a `.NET` tutorial, confirm that `dotnet` is available:

```bash
dotnet --info
```

Choose the target framework that the project tutorial standardizes on, for example `net10.0`.

## Suggested Scaffold Commands

For a core-library repo, use placeholders and substitute the values that fit the project you are building:

```bash
dotnet new sln --format sln --name <solution-name> --output <solution-root>
dotnet new classlib --name <library-name> --output <solution-root>/src/<library-name> --framework <target-framework>
dotnet sln <solution-root>/<solution-name>.sln add <solution-root>/src/<library-name>/<library-name>.csproj
```

## Suggested File Shape

```text
<solution-root>/
  <solution-name>.sln
  src/
    <library-name>/
      <library-name>.csproj
```

If the class-library template creates a placeholder file such as `Class1.cs`, replace it with the production file names used by the tutorial.

For an adapter repo, use this guide for the base `.NET` CLI conventions, then switch to the relevant adapter setup guide for the actual host project scaffold.

## Ready State

For a core-library repo, the base `.NET` toolchain setup is ready when:

- `dotnet` commands run successfully
- the solution exists
- the production class library exists
- the class library is added to the solution
