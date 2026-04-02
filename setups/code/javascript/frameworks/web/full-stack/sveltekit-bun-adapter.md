<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../../../README.md) / [Setups](../../../../../README.md) / [Code](../../../../README.md) / [javascript](../../../README.md) / [Frameworks](../../README.md) / [Web](../README.md) / [Full Stack](README.md) / sveltekit-bun-adapter.md
<!-- breadcrumbs:end -->

# SvelteKit + Bun Adapter

Reusable JavaScript `web/full-stack` framework guidance for adapters that use SvelteKit with a Bun adapter for compiler-first, server-rendered web applications.

## Use This After

- [Toolchain](../../../toolchain.md)
- one guide from [Testing](../../../testing/README.md) when one exists
- [Full Stack](../../../adapters/web/full-stack.md)

## Adapter Rule

A SvelteKit + Bun adapter should stay thin:

- translate web input into calls to the core library
- translate core results into rendered pages, server actions, or reactive UI state
- keep validation and service decisions in the core project logic

## Ready State

The SvelteKit + Bun adapter setup is ready when:

- the adapter repo already follows the generic `web/full-stack` setup
- SvelteKit- and Bun-specific host code delegates to the core library
- project rules remain in the separately tested core library repo
