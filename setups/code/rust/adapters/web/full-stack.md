<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../../../README.md) / [Setups](../../../../../README.md) / [Code](../../../../README.md) / [rust](../../../README.md) / [Adapters](../../README.md) / [Web](../README.md) / full-stack.md
<!-- breadcrumbs:end -->

# Full Stack

Reusable Rust adapter setup for the `web/full-stack` target.

## Goal

Create a thin web host that can receive requests, render UI, and delegate project rules to a well-tested core library.

## Use This After

- [Toolchain](../../../toolchain.md)
- [Libtest](../../../testing/libtest.md)
- an existing core library repo for the project logic
- one guide from [Frameworks](../../../frameworks/README.md) when the chosen stack needs framework-specific bootstrap instructions

## Scope

This guide covers:

- wiring a Rust web host repo to a core library
- adding a dependency on the core library from the host
- keeping UI, transport, and host lifecycle concerns outside the core project rules

This guide does not cover:

- choosing a specific Rust web framework
- project-specific routes, pages, or components
- project-specific persistence, auth, or realtime behavior

## Suggested Integration Steps

1. Use the chosen framework guide to scaffold the web host project.
2. Add a dependency from the web host to the core library.
3. Keep request handlers, components, and server actions thin.
4. Delegate validation and service rules to the core library.
5. Let the web host focus on transport, rendering, and user interaction.

## Adapter Rule

The Rust `web/full-stack` adapter should stay thin:

- translate web input into calls to the core library
- translate core results into rendered output or response models
- keep validation and service decisions in the core project logic

## Ready State

The Rust `web/full-stack` adapter setup is ready when:

- the web host repo exists
- the web host depends on the core library
- the web host can delegate project rules to the core library instead of redefining them
