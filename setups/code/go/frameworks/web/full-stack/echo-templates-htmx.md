<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../../../README.md) / [Setups](../../../../../README.md) / [Code](../../../../README.md) / [go](../../../README.md) / [Frameworks](../../README.md) / [Web](../README.md) / [Full Stack](README.md) / echo-templates-htmx.md
<!-- breadcrumbs:end -->

# Echo + Templates + HTMX

Reusable Go `web/full-stack` framework guidance for adapters that use Echo + templates + HTMX for server-rendered, middleware-driven web interfaces.

## Use This After

- [Toolchain](../../../toolchain.md)
- one guide from [Testing](../../../testing/README.md) when one exists
- [Full Stack](../../../adapters/web/full-stack.md)

## Adapter Rule

An Echo + templates + HTMX adapter should stay thin:

- translate web input into calls to the core library
- translate core results into rendered templates, response models, or incremental UI updates
- keep validation and service decisions in the core project logic

## Ready State

The Echo + templates + HTMX setup is ready when:

- the adapter repo already follows the generic `web/full-stack` setup
- Echo-, template-, and HTMX-specific host code delegates to the core library
- project rules remain in the separately tested core library repo
