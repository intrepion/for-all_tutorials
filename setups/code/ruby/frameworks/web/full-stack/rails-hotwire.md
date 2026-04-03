<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../../../README.md) / [Setups](../../../../../README.md) / [Code](../../../../README.md) / [ruby](../../../README.md) / [Frameworks](../../README.md) / [Web](../README.md) / [Full Stack](README.md) / rails-hotwire.md
<!-- breadcrumbs:end -->

# Rails + Hotwire

Reusable Ruby `web/full-stack` framework guidance for adapters that use Rails + Hotwire for server-rendered, progressively enhanced web interfaces.

## Use This After

- [Toolchain](../../../toolchain/README.md)
- one guide from [Testing](../../../testing/README.md) when one exists
- [Full Stack](../../../adapters/web/full-stack.md)

## Adapter Rule

A Rails + Hotwire adapter should stay thin:

- translate requests, forms, or stream events into calls to the core library
- translate core results into rendered views, Turbo responses, or component state
- keep validation and service decisions in the core project logic

## Ready State

The Rails + Hotwire setup is ready when:

- the adapter repo already follows the generic `web/full-stack` setup
- Rails- and Hotwire-specific host code delegates to the core library
- project rules remain in the separately tested core library repo
