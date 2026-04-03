<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../../../README.md) / [Setups](../../../../../README.md) / [Code](../../../../README.md) / [rust](../../../README.md) / [Frameworks](../../README.md) / [Web](../README.md) / [Full Stack](README.md) / leptos-ssr.md
<!-- breadcrumbs:end -->

# Leptos SSR

Reusable Rust `web/full-stack` framework guidance for adapters that use Leptos SSR for server-rendered, reactive web interfaces.

## Use This After

- [Toolchain](../../../toolchain/README.md)
- [Libtest](../../../testing/libtest.md)
- [Full Stack](../../../adapters/web/full-stack.md)

## Scope

This guide covers:

- using Leptos SSR as the full-stack web host for an adapter repo
- keeping framework-specific routing, rendering, and streaming concerns inside the adapter
- mapping user input and rendered output cleanly to the core library contract

This guide does not cover:

- project-specific business rules
- project-specific routes, persistence, auth, or deployment
- moving validation or service logic out of the core library

## Adapter Rule

A Leptos SSR adapter should stay thin:

- translate web input into calls to the core library
- translate core results into rendered output, component state, or streamed UI updates
- keep validation and service decisions in the core project logic

## Ready State

The Leptos SSR setup is ready when:

- the adapter repo already follows the generic `web/full-stack` setup
- Leptos-specific host and component code delegates to the core library
- project rules remain in the separately tested core library repo
