<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../README.md) / [Setups](../../../README.md) / [Code](../../README.md) / [dotnet](../README.md) / [Testing](README.md) / tunit.md
<!-- breadcrumbs:end -->

# TUnit

Reusable TUnit setup guide for `.NET` tutorials.

## Goal

Create a TUnit test project, attach it to a `.NET` solution, and use Microsoft.Testing.Platform-compatible coverage collection.

## Option A: Install The Template

If you want the official template workflow:

```bash
dotnet new install TUnit.Templates
dotnet new TUnit --name <test-project-name> --output <test-project-path>
dotnet sln <solution-path> add <test-project-path>/<test-project-name>.csproj
dotnet add <test-project-path>/<test-project-name>.csproj reference <library-project-path>
```

## Option B: Manual Setup

If you prefer a package-based setup:

```bash
dotnet new console --name <test-project-name> --output <test-project-path> --framework <target-framework>
dotnet add <test-project-path>/<test-project-name>.csproj package TUnit
dotnet sln <solution-path> add <test-project-path>/<test-project-name>.csproj
dotnet add <test-project-path>/<test-project-name>.csproj reference <library-project-path>
```

Then remove the generated `Program.cs`, because TUnit handles the entry point for the test project.

Do not add `Microsoft.NET.Test.Sdk`, `coverlet.collector`, or `coverlet.msbuild` to a TUnit project.

## Minimal Test Shape

```csharp
namespace Example.Tests;

public sealed class ExampleTests
{
    [Test]
    public async Task Example_Passes()
    {
        await Assert.That(2 + 2).IsEqualTo(4);
    }
}
```

## Coverage

Use TUnit's Microsoft.Testing.Platform-compatible coverage flow:

```bash
dotnet test <test-project-path>/<test-project-name>.csproj -c Release -- --coverage
```

## Official References

- [Installing TUnit](https://tunit.dev/docs/getting-started/installation/)
- [Running your tests](https://tunit.dev/docs/getting-started/running-your-tests/)
