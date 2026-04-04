---
partial_kind: adapter-partial
ecosystem: dotnet
surface: web-service
target: all
---

# All

Reusable `.NET` adapter setup for the `web-service/all` target.

## Goal

Create a thin HTTP service adapter that can expose endpoints, delegate payload building and formatting to a well-tested core library, and keep service transport concerns out of the core.

## Use This After

- [Toolchain](../../../toolchain/README.md)
- one guide from [Testing](../../../testing/README.md)
- an existing core library repo for the project logic
- one guide from [Frameworks](../../../frameworks/README.md) when the chosen stack needs service-bootstrap instructions

## Scope

This guide covers:

- scaffolding a small service host repo
- adding the service project to its solution
- adding a dependency on the core library from the service host
- keeping HTTP endpoint, status code, and clock concerns outside the core project rules

This guide does not cover:

- choosing a specific .NET service framework
- project-specific auth, persistence, or deployment
- project-specific API versioning or background jobs

## Suggested Integration Steps

1. Scaffold the service host project with the smallest workable HTTP shape for the chosen run.
2. Add the service host project to the solution.
3. Add a dependency from the service host to the core library.
4. Keep endpoint handlers thin.
5. Delegate payload building, parsing, formatting, and domain rules to the core library.
6. Let the service host own only transport and host lifecycle concerns.

## Suggested File Shape

```text
<solution-root>/
  <solution-name>.sln
  src/
    <service-name>/
      <service-name>.csproj
      Program.cs
```

The exact service files will vary, but the service repo should depend on the core library instead of duplicating it.

## Adapter Rule

The `web-service/all` adapter should stay thin:

- receive HTTP requests
- gather host-owned values such as the current time when the project requires them
- pass those values into the core library
- translate core results into HTTP responses

## Ready State

The `.NET` `web-service/all` adapter setup is ready when:

- the service project exists
- the service project is added to the solution
- the service project depends on the core library
- the service host delegates payload building or formatting to the core library
