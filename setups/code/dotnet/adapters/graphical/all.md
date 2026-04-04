<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../../../README.md) / [Setups](../../../../../README.md) / [Code](../../../../README.md) / [dotnet](../../../README.md) / [Adapters](../../README.md) / [Graphical](README.md) / all.md
<!-- breadcrumbs:end -->

# All

Reusable `.NET` adapter setup for the `graphical/all` target.

## Goal

Create a thin graphical adapter that can collect user input, fetch or compute results, and render those results visually while delegating project rules to a well-tested core library.

## Use This After

- [Toolchain](../../../toolchain/README.md)
- one guide from [Testing](../../../testing/README.md)
- an existing core library repo for the project logic
- one guide from [Frameworks](../../../frameworks/README.md) when the chosen stack needs graphical-bootstrap instructions

## Scope

This guide covers:

- scaffolding a small graphical app repo
- adding the graphical project to its solution
- adding a dependency on the core library from the graphical app
- keeping HTTP, image loading, and rendering concerns outside the core project rules

This guide does not cover:

- choosing a specific desktop or UI framework
- project-specific window layouts or visual design
- project-specific persistence, auth, or caching behavior

## Suggested Integration Steps

1. Scaffold the graphical app with the smallest workable UI host for the chosen run.
2. Add the graphical app project to the solution.
3. Add a dependency from the graphical app to the core library.
4. Keep UI events, image loading, and rendering code thin.
5. Delegate parsing, validation, formatting, and domain rules to the core library.
6. Let the graphical layer own only presentation and user interaction.

## Suggested File Shape

```text
<solution-root>/
  <solution-name>.sln
  src/
    <app-name>/
      <app-name>.csproj
      Program.cs
```

The exact UI files will vary, but the graphical app repo should depend on the core library instead of duplicating it.

## Adapter Rule

The `graphical/all` adapter should stay thin:

- collect input through the graphical surface
- call external services when the project requires them
- pass fetched or raw values into the core library
- render the returned core results without redefining the project rules

## Ready State

The `.NET` `graphical/all` adapter setup is ready when:

- the graphical app project exists
- the graphical app project is added to the solution
- the graphical app project depends on the core library
- the graphical layer delegates parsing or display-item construction to the core library
