<!-- breadcrumbs:start -->
[for-all_tutorials](../../../README.md) / [Setups](../../README.md) / [C#](../README.md) / .NET SDK
<!-- breadcrumbs:end -->

# .NET SDK

Base C# and .NET setup guidance for project tutorials that use the `dotnet` toolchain.

## Goal

Set up a clean, repeatable foundation for C# tutorials without tying the instructions to any one local machine.

## Scope

This guide covers:

- the base `dotnet` CLI workflow
- solution and class-library scaffolding
- common repository conventions for C# tutorials

This guide does not cover:

- test-framework-specific scaffolding
- UI or web framework bootstrapping
- project-specific business logic

## Recommended Conventions

- use SDK-style projects
- keep source under `src/`
- keep tests under `tests/`
- keep nullable reference types enabled
- keep implicit usings enabled unless a tutorial has a strong reason not to

## Verify The Toolchain

Before starting a C# tutorial, confirm that `dotnet` is available:

```bash
dotnet --info
```

Choose the target framework that the project tutorial standardizes on, for example `net10.0`.

## Suggested Scaffold Commands

Use placeholders and substitute the values that fit the project you are building:

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

## Ready State

The base C# SDK setup is ready when:

- `dotnet` commands run successfully
- the solution exists
- the production class library exists
- the class library is added to the solution
