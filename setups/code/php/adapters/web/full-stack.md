<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../../../README.md) / [Setups](../../../../../README.md) / [Code](../../../../README.md) / [php](../../../README.md) / [Adapters](../../README.md) / [Web](../README.md) / full-stack.md
<!-- breadcrumbs:end -->

# Full Stack

Reusable PHP adapter setup for the `web/full-stack` target.

## Goal

Create a thin web host that can receive requests, render UI, and delegate project rules to a well-tested core library.

## Use This After

- [Toolchain](../../../toolchain.md)
- an existing core library repo for the project logic
- one guide from [Testing](../../../testing/README.md) when a PHP testing guide has been added
- one guide from [Frameworks](../../../frameworks/README.md) when the chosen stack needs framework-specific bootstrap instructions

## Adapter Rule

The PHP `web/full-stack` adapter should stay thin:

- translate web input into calls to the core library
- translate core results into rendered output or reactive UI state
- keep validation and service decisions in the core project logic

## Ready State

The PHP `web/full-stack` adapter setup is ready when:

- the web host repo exists
- the web host depends on the core library
- the web host can delegate project rules to the core library instead of redefining them
