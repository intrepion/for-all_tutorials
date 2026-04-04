---
partial_kind: testing-partial
ecosystem: dotnet
testing: xunit
---

# xUnit

Reusable xUnit setup partial for `.NET` tutorials.

## Goal

Create an xUnit test project, attach it to a `.NET` solution, and enable repeatable coverage collection.

## Create The Test Project

Use the exact project names and paths from the compiled tutorial:

```bash
dotnet new xunit --name <test-project-name> --output <test-project-path>
dotnet sln <solution-path> add <test-project-path>/<test-project-name>.csproj
dotnet add <test-project-path>/<test-project-name>.csproj reference <library-project-path>
```

If the tutorial needs a non-default target framework, add `--framework <target-framework>` to both the library scaffold command and the `dotnet new xunit` command.

Then replace the template placeholder file such as `UnitTest1.cs` with the test file names declared by the tutorial.

## Minimal Test Shape

```csharp
namespace Example.Tests;

public sealed class ExampleTests
{
    [Fact]
    public void Example_Passes()
    {
        Assert.Equal(4, 2 + 2);
    }
}
```

## Coverage

Use this coverage-friendly command:

```bash
dotnet test <solution-path> --collect:"XPlat Code Coverage"
```

If your generated project does not already include `coverlet.collector`, add it before collecting coverage.
