---
partial_kind: adapter-partial
ecosystem: dotnet
surface: client
target: all
---

# All

Reusable `.NET` adapter setup for the `client/all` target.

## Goal

Create a thin client adapter that can call an HTTP service, delegate parsing and formatting to a well-tested core library, and render the returned result.

## Use This After

- [Toolchain](../../../toolchain/README.md)
- one guide from [Testing](../../../testing/README.md)
- an existing core library repo for the project logic
- one guide from [Frameworks](../../../frameworks/README.md) when the chosen stack needs client-bootstrap instructions

## Scope

This guide covers:

- scaffolding a small client app repo
- adding the client project to its solution
- adding a dependency on the core library from the client
- keeping HTTP transport and presentation concerns outside the core project rules

This guide does not cover:

- choosing a specific client UI technology
- project-specific routes, pages, or widgets
- project-specific persistence, auth, or polling behavior

## Suggested Integration Steps

1. Scaffold the client host project with the smallest workable .NET app shape for the chosen run.
2. Add the client project to the solution.
3. Add a dependency from the client project to the core library.
4. Keep HTTP request code thin and isolated from the core logic.
5. Delegate response parsing, formatting, and domain rules to the core library.
6. Let the client own only transport, rendering, and user interaction.

## Suggested File Shape

```text
<solution-root>/
  <solution-name>.sln
  src/
    <client-name>/
      <client-name>.csproj
      Program.cs
```

The exact client files will vary by run, but the client repo should depend on the core library instead of duplicating it.

## Adapter Rule

The `client/all` adapter should stay thin:

- call the external or companion service
- pass service responses into the core library
- render the returned core result without redefining the project rules

## Ready State

The `.NET` `client/all` adapter setup is ready when:

- the client project exists
- the client project is added to the solution
- the client project depends on the core library
- the client can call the target service and delegate parsing or formatting to the core library
