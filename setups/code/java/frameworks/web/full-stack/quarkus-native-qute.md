<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../../../README.md) / [Setups](../../../../../README.md) / [Code](../../../../README.md) / [java](../../../README.md) / [Frameworks](../../README.md) / [Web](../README.md) / [Full Stack](README.md) / quarkus-native-qute.md
<!-- breadcrumbs:end -->

# Quarkus (Native) + Qute

Reusable Java `web/full-stack` framework guidance for adapters that use Quarkus + Qute for native-friendly, server-rendered web applications.

## Use This After

- [Toolchain](../../../toolchain/README.md)
- one guide from [Testing](../../../testing/README.md) when one exists
- [Full Stack](../../../adapters/web/full-stack.md)

## Adapter Rule

A Quarkus + Qute adapter should stay thin:

- translate web input into calls to the core library
- translate core results into rendered templates, view models, or reactive server responses
- keep validation and service decisions in the core project logic

## Ready State

The Quarkus + Qute setup is ready when:

- the adapter repo already follows the generic `web/full-stack` setup
- Quarkus- and Qute-specific host code delegates to the core library
- project rules remain in the separately tested core library repo
