---
partial_kind: testing-partial
ecosystem: dotnet
testing: tunit
---

# TUnit

Reusable TUnit setup partial for `.NET` tutorials.

## Goal

Create a TUnit test project, attach it to a `.NET` solution, and use Microsoft.Testing.Platform-compatible coverage collection.

## Create The Test Project

Install the official TUnit template first:

```bash
dotnet new install TUnit.Templates
dotnet new TUnit --name <test-project-name> --output <test-project-path>
dotnet sln <solution-path> add <test-project-path>/<test-project-name>.csproj
dotnet add <test-project-path>/<test-project-name>.csproj reference <library-project-path>
```

If the tutorial needs a non-default target framework, update the generated project file after scaffolding.

Then remove the starter example files that `dotnet new TUnit` generated, such as `BasicTests.cs`, `Calculator.cs`, `DataDrivenTests.cs`, `DependencyInjectionTests.cs`, and `HooksAndLifecycle.cs`, and replace them with the test file names declared by the tutorial.

## Minimal Test Shape

```csharp
using System.Threading.Tasks;
using TUnit.Assertions;
using TUnit.Assertions.Extensions;
using TUnit.Core;

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
