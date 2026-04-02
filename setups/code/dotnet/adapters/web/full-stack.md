<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../../../README.md) / [Setups](../../../../../README.md) / [Code](../../../../README.md) / [dotnet](../../../README.md) / [Adapters](../../README.md) / [Web](../README.md) / full-stack.md
<!-- breadcrumbs:end -->

# Full Stack

Reusable `.NET` adapter setup for the `web/full-stack` target.

## Goal

Create a thin web host that can receive requests, render UI, and delegate project rules to a well-tested core library.

## Use This After

- [Toolchain](../../../toolchain.md)
- one guide from [Testing](../../../testing/README.md)
- an existing core library repo for the project logic
- one guide from [Frameworks](../../../frameworks/README.md) when the chosen stack needs framework-specific bootstrap instructions

## Scope

This guide covers:

- wiring a `.NET` web host repo to a core library
- adding the host project to its solution
- adding a dependency on the core library from the host
- keeping UI, transport, and host lifecycle concerns outside the core project rules

This guide does not cover:

- choosing a specific `.NET` web framework
- project-specific routes, pages, or components
- project-specific persistence, auth, or realtime behavior

## Suggested Integration Steps

1. Use the chosen framework guide to scaffold the web host project.
2. Add the web host project to the solution.
3. Add a dependency from the web host to the core library.
4. Keep request handlers, page models, components, or server actions thin.
5. Delegate validation and service rules to the core library.
6. Let the web host focus on transport, rendering, and user interaction.

## Suggested File Shape

```text
<solution-root>/
  <solution-name>.sln
  src/
    <host-name>/
      <host-name>.csproj
      Program.cs
```

The exact host files will vary by framework. Keep the core library in its own repo and keep that dependency stable even when the host framework changes.

## Adapter Rule

The `web/full-stack` adapter should stay thin:

- translate web input into calls to the core library
- translate core results into rendered output or response models
- keep validation and service decisions in the core project logic

## Ready State

The `.NET` `web/full-stack` adapter setup is ready when:

- the web host project exists
- the web host project is added to the solution
- the web host depends on the core library
- the web host can delegate project rules to the core library instead of redefining them
