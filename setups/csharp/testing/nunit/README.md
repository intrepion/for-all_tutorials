<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../README.md) / [Setups](../../../README.md) / [C#](../../README.md) / [Testing](../README.md) / NUnit
<!-- breadcrumbs:end -->

# NUnit

Reusable NUnit setup guide for C# tutorials.

## Goal

Create an NUnit test project, attach it to a C# solution, and enable repeatable coverage collection.

## Create The Test Project

Substitute the placeholders with the names and paths from the project tutorial:

```bash
dotnet new nunit --name <test-project-name> --output <test-project-path> --framework <target-framework>
dotnet sln <solution-path> add <test-project-path>/<test-project-name>.csproj
dotnet add <test-project-path>/<test-project-name>.csproj reference <library-project-path>
```

Then replace the template placeholder file such as `UnitTest1.cs` with the test file names used by the tutorial.

## Minimal Test Shape

```csharp
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

If your generated project does not already include `coverlet.collector`, add it before collecting coverage.
