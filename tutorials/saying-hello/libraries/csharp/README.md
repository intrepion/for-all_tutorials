<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../README.md) / [Tutorials](../../../README.md) / [Saying Hello](../../README.md) / [Libraries](../README.md) / C#
<!-- breadcrumbs:end -->

# C#

Spec-driven, test-driven C# library tutorial for the `saying-hello` core `greet` contract.

## Goal

Build the smallest C# library that satisfies the canonical `saying-hello` spec before any surface adapter is added.

This tutorial should produce a tiny, well-tested core that later surface tutorials can call instead of re-implementing greeting rules in UI or transport code.

## Start From The Spec

Use [specs/saying-hello/README.md](../../../../specs/saying-hello/README.md) as the source of truth.

The library contract is:

```text
greet(name: string) -> string
```

Canonical behavior:

- trim leading and trailing whitespace from `name`
- return `Hello, <name>!` when the trimmed value is non-empty
- return `Hello!` when the trimmed value is empty or whitespace only

## Prerequisites

- .NET SDK `10.0.201` or later in the local environment
- comfort with basic `dotnet` CLI commands
- willingness to follow red, green, refactor instead of writing the library first

## Recommended Project Shape

Create a small solution with one class library and one test project:

```text
SayingHello.CSharp/
  SayingHello.sln
  src/
    SayingHello/
      SayingHello.csproj
      GreetingService.cs
  tests/
    SayingHello.Tests/
      SayingHello.Tests.csproj
      GreetingServiceTests.cs
```

Recommended commands:

```bash
dotnet new sln --format sln --name SayingHello --output SayingHello.CSharp
dotnet new classlib --name SayingHello --output SayingHello.CSharp/src/SayingHello --framework net10.0
dotnet new xunit --name SayingHello.Tests --output SayingHello.CSharp/tests/SayingHello.Tests --framework net10.0
dotnet sln SayingHello.CSharp/SayingHello.sln add SayingHello.CSharp/src/SayingHello/SayingHello.csproj
dotnet sln SayingHello.CSharp/SayingHello.sln add SayingHello.CSharp/tests/SayingHello.Tests/SayingHello.Tests.csproj
dotnet add SayingHello.CSharp/tests/SayingHello.Tests/SayingHello.Tests.csproj reference SayingHello.CSharp/src/SayingHello/SayingHello.csproj
```

Using `--format sln` keeps the solution file familiar for early tutorials.

After scaffolding, remove the placeholder files that the templates generate and replace them with the names used in this tutorial:

- delete `src/SayingHello/Class1.cs`
- delete `tests/SayingHello.Tests/UnitTest1.cs`
- add `src/SayingHello/GreetingService.cs`
- add `tests/SayingHello.Tests/GreetingServiceTests.cs`

## Work Test-First

Follow this loop for every slice:

1. read the spec and choose one behavior
2. write the smallest failing test for that behavior
3. make the test compile
4. implement the smallest change that makes it pass
5. refactor only after the test is green
6. review coverage before moving on

Do not write the full `greet` method up front. Let the spec unfold through tests.

## First Failing Test

Start with the happiest path:

```csharp
using SayingHello;

namespace SayingHello.Tests;

public sealed class GreetingServiceTests
{
    [Fact]
    public void Greet_ReturnsGreetingForNonEmptyName()
    {
        var service = new GreetingService();

        var result = service.Greet("Ada");

        Assert.Equal("Hello, Ada!", result);
    }
}
```

Your first `dotnet test` run should fail because `GreetingService` does not exist yet. That is the right starting point.

## First Green Implementation

Create the smallest implementation that satisfies the first test:

```csharp
namespace SayingHello;

public sealed class GreetingService
{
    public string Greet(string name)
    {
        return $"Hello, {name}!";
    }
}
```

This is intentionally incomplete. It gets one behavior green without pretending the rest of the spec is already handled.

## Expand The Test Suite

Add the remaining spec cases one by one:

- `Greet_ReturnsGreetingForTrimmedName`
- `Greet_ReturnsGenericGreetingForEmptyString`
- `Greet_ReturnsGenericGreetingForWhitespaceOnlyString`

Example trimming test:

```csharp
[Fact]
public void Greet_ReturnsGreetingForTrimmedName()
{
    var service = new GreetingService();

    var result = service.Greet("  Ada  ");

    Assert.Equal("Hello, Ada!", result);
}
```

Only after the trimming and empty-input tests are red should you refactor the implementation to something like:

```csharp
namespace SayingHello;

public sealed class GreetingService
{
    public string Greet(string name)
    {
        var trimmedName = name.Trim();

        if (trimmedName.Length == 0)
        {
            return "Hello!";
        }

        return $"Hello, {trimmedName}!";
    }
}
```

## Coverage Expectations

This library tutorial should meet the repo-wide baseline:

- `90%` code coverage
- `85%` branch coverage

For this project, the core `greet` logic is service logic, so it should meet the stricter standard:

- `100%` code coverage
- `100%` branch coverage

That means the tests should fully exercise:

- non-empty input
- trimmed input
- empty input
- whitespace-only input

## Suggested Test Commands

Run the test suite often:

```bash
dotnet test SayingHello.CSharp/SayingHello.sln
```

The current xUnit template in this workspace includes `coverlet.collector` by default, so a concrete coverage command can be:

```bash
dotnet test SayingHello.CSharp/SayingHello.sln --collect:"XPlat Code Coverage"
```

The important requirement is to verify both the repo's `90/85` baseline and the `100/100` goal for service logic.

## Definition Of Done

This tutorial is done when:

- the library behavior matches [specs/saying-hello/README.md](../../../../specs/saying-hello/README.md)
- the tutorial clearly follows a red, green, refactor sequence
- the core `GreetingService.Greet` behavior is thoroughly tested
- the library achieves `100%` code coverage and `100%` branch coverage for the service logic
- a later surface tutorial can consume this library instead of rewriting the greeting rules

## Next Step

After this library tutorial is complete, the natural follow-up is a surface tutorial such as:

- `command-line/all` in C#
- `web/full-stack` in C#

Those tutorials should treat this library as the source of core behavior and keep the adapter layer thin.
