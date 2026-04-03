<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../../../README.md) / [Setups](../../../../../README.md) / [Code](../../../../README.md) / [beam](../../../README.md) / [Frameworks](../../README.md) / [Web](../README.md) / [Full Stack](README.md) / phoenix-liveview.md
<!-- breadcrumbs:end -->

# Phoenix + LiveView

Reusable BEAM `web/full-stack` framework guidance for adapters that use Phoenix + LiveView for server-rendered, reactive web interfaces.

## Use This After

- [Toolchain](../../../toolchain/README.md)
- one guide from [Testing](../../../testing/README.md) when one exists
- [Full Stack](../../../adapters/web/full-stack.md)

## Adapter Rule

A Phoenix + LiveView adapter should stay thin:

- translate requests, form input, or websocket events into calls to the core library
- translate core results into rendered templates, component state, or pushed updates
- keep validation and service decisions in the core project logic

## Ready State

The Phoenix + LiveView setup is ready when:

- the adapter repo already follows the generic `web/full-stack` setup
- Phoenix- and LiveView-specific host code delegates to the core library
- project rules remain in the separately tested core library repo
