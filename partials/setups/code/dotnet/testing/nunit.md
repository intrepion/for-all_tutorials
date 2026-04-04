---
partial_kind: testing-partial
ecosystem: dotnet
testing: nunit
---

# NUnit

Reusable NUnit setup partial for `.NET` tutorials.

## Goal

Create an NUnit test project, attach it to a `.NET` solution, and enable repeatable coverage collection.

## Create The Test Project

Use the exact project names and paths from the compiled tutorial:

```bash
dotnet new nunit --name <test-project-name> --output <test-project-path>
dotnet sln <solution-path> add <test-project-path>/<test-project-name>.csproj
dotnet add <test-project-path>/<test-project-name>.csproj reference <library-project-path>
```

If the tutorial needs a non-default target framework, add `--framework <target-framework>` to both the library scaffold command and the `dotnet new nunit` command.

Then replace the template placeholder file such as `UnitTest1.cs` with the test file names declared by the tutorial.

## Minimal Test Shape

```csharp
using NUnit.Framework;

namespace Example.Tests;

public sealed class ExampleTests
{
    [Test]
    public void Example_Passes()
    {
        Assert.That(2 + 2, Is.EqualTo(4));
    }
}
```

## Coverage

Use this coverage-friendly command:

```bash
dotnet test <solution-path> --collect:"XPlat Code Coverage"
```
