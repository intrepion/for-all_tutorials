---
partial_kind: testing-partial
ecosystem: dotnet
testing: mstest
---

# MSTest

Reusable MSTest setup partial for `.NET` tutorials.

## Goal

Create an MSTest test project, attach it to a `.NET` solution, and keep coverage collection aligned with the repo policy.

## Create The Test Project

Use the exact project names and paths from the compiled tutorial:

```bash
dotnet new mstest --name <test-project-name> --output <test-project-path>
dotnet sln <solution-path> add <test-project-path>/<test-project-name>.csproj
dotnet add <test-project-path>/<test-project-name>.csproj reference <library-project-path>
```

If the tutorial needs a non-default target framework, add `--framework <target-framework>` to both the library scaffold command and the `dotnet new mstest` command.

Then replace the template placeholder file such as `UnitTest1.cs` or `Test1.cs` with the test file names declared by the tutorial.

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
