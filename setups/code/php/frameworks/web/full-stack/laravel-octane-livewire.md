<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../../../README.md) / [Setups](../../../../../README.md) / [Code](../../../../README.md) / [php](../../../README.md) / [Frameworks](../../README.md) / [Web](../README.md) / [Full Stack](README.md) / laravel-octane-livewire.md
<!-- breadcrumbs:end -->

# Laravel (Octane) + Livewire

Reusable PHP `web/full-stack` framework guidance for adapters that use Laravel + Livewire with Octane-style hosting for server-rendered, reactive web interfaces.

## Use This After

- [Toolchain](../../../toolchain/README.md)
- one guide from [Testing](../../../testing/README.md) when one exists
- [Full Stack](../../../adapters/web/full-stack.md)

## Adapter Rule

A Laravel + Livewire adapter should stay thin:

- translate web input into calls to the core library
- translate core results into rendered views, component state, or pushed updates
- keep validation and service decisions in the core project logic

## Ready State

The Laravel + Livewire setup is ready when:

- the adapter repo already follows the generic `web/full-stack` setup
- Laravel- and Livewire-specific host code delegates to the core library
- project rules remain in the separately tested core library repo
