<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../README.md) / [Setups](../../../README.md) / [C#](../../README.md) / [Testing](../README.md) / MSTest
<!-- breadcrumbs:end -->

# MSTest

Reusable MSTest setup guide for C# tutorials.

## Goal

Create an MSTest test project, attach it to a C# solution, and keep coverage collection aligned with the repo policy.

## Create The Test Project

Substitute the placeholders with the names and paths from the project tutorial:

```bash
dotnet new mstest --name <test-project-name> --output <test-project-path> --framework <target-framework> --coverage-tool coverlet
dotnet sln <solution-path> add <test-project-path>/<test-project-name>.csproj
dotnet add <test-project-path>/<test-project-name>.csproj reference <library-project-path>
```

The explicit `--coverage-tool coverlet` keeps the coverage workflow aligned with the other C# test-framework setups.

Then replace the template placeholder file such as `Test1.cs` with the test file names used by the tutorial.

## Minimal Test Shape

```csharp
using Microsoft.VisualStudio.TestTools.UnitTesting;

namespace Example.Tests;

[TestClass]
public sealed class ExampleTests
{
    [TestMethod]
    public void Example_Passes()
    {
        Assert.AreEqual(4, 2 + 2);
    }
}
```

## Coverage

Use this coverage-friendly command:

```bash
dotnet test <solution-path> --collect:"XPlat Code Coverage"
```
