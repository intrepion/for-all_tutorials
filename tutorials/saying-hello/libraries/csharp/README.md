<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../README.md) / [Tutorials](../../../README.md) / [Saying Hello](../../README.md) / [Libraries](../README.md) / C#
<!-- breadcrumbs:end -->

# C#

Spec-driven C# library tutorial for the `saying-hello` core `greet` contract.

## Goal

Build the smallest C# library that satisfies the canonical `saying-hello` spec before any surface adapter is added.

This tutorial should produce a tiny core that later surface tutorials can call instead of re-implementing greeting rules in UI or transport code.

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

## Related Setups

Complete the reusable setup guides before or alongside this tutorial:

- [C# Setup Root](../../../../setups/csharp/README.md)
- [.NET SDK](../../../../setups/csharp/sdk/README.md)
- [Testing](../../../../setups/csharp/testing/README.md)
- [xUnit](../../../../setups/csharp/testing/xunit/README.md)
- [NUnit](../../../../setups/csharp/testing/nunit/README.md)
- [MSTest](../../../../setups/csharp/testing/mstest/README.md)
- [TUnit](../../../../setups/csharp/testing/tunit/README.md)

## Prerequisites

- the shared C# setup guides are understood well enough to scaffold a library and a test project
- comfort with basic `dotnet` CLI commands
- willingness to follow red, green, refactor through one chosen testing setup instead of writing the library first

## Shared Project Shape

Create a small solution with one class library and one test project:

```text
SayingHello.CSharp/
  SayingHello.sln
  src/
    SayingHello/
      SayingHello.csproj
      GreetingService.cs
  tests/
    <chosen-test-project>/
      <chosen-test-project>.csproj
      GreetingServiceTests.cs
```

Shared scaffold commands:

```bash
dotnet new sln --format sln --name SayingHello --output SayingHello.CSharp
dotnet new classlib --name SayingHello --output SayingHello.CSharp/src/SayingHello --framework <target-framework>
dotnet sln SayingHello.CSharp/SayingHello.sln add SayingHello.CSharp/src/SayingHello/SayingHello.csproj
```

Using `--format sln` keeps the solution file familiar for early tutorials. The chosen testing setup will add the test project afterwards.

After scaffolding, remove the placeholder files that the templates generate and replace them with the names used in this tutorial:

- delete `src/SayingHello/Class1.cs`
- add `src/SayingHello/GreetingService.cs`

## Choose A Testing Setup

Pick one test framework and follow its setup guide:

- [xUnit](../../../../setups/csharp/testing/xunit/README.md)
- [NUnit](../../../../setups/csharp/testing/nunit/README.md)
- [MSTest](../../../../setups/csharp/testing/mstest/README.md)
- [TUnit](../../../../setups/csharp/testing/tunit/README.md)

Each setup guide explains how to scaffold the test project, attach it to the solution, and collect coverage. This project tutorial supplies the production target and the project-specific behaviors that those tests must cover.

## Shared Implementation Target

The production code should converge on a tiny service like this after the tests drive it there:

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

Do not start by typing the final version. Let the chosen testing setup and the project spec pull the implementation into shape through failing tests.

## Coverage Expectations

Whichever testing setup you choose, the resulting tutorial should meet the repo-wide baseline:

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

## Definition Of Done

This tutorial is done when:

- the library behavior matches [specs/saying-hello/README.md](../../../../specs/saying-hello/README.md)
- one testing setup clearly supports a red, green, refactor sequence
- the core `GreetingService.Greet` behavior is thoroughly tested
- the library achieves `100%` code coverage and `100%` branch coverage for the service logic
- a later surface tutorial can consume this library instead of rewriting the greeting rules

## Next Step

After this library tutorial is complete, the natural follow-up is a surface tutorial such as:

- `command-line/all` in C#
- `web/full-stack` in C#

Those tutorials should treat this library as the source of core behavior and keep the adapter layer thin.
