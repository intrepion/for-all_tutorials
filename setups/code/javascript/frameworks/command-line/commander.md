<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../../README.md) / [Setups](../../../../README.md) / [Code](../../../README.md) / [javascript](../../README.md) / [Frameworks](../README.md) / [Command Line](README.md) / commander.md
<!-- breadcrumbs:end -->

# Commander

Reusable JavaScript command-line framework guidance for adapters that use Commander for argument parsing and command structure.

## Use This After

- [Toolchain](../../toolchain/README.md)
- [All](../../adapters/command-line/all.md)

## Guidance

Choose this option when the adapter should use Commander for command definitions, options, and argument parsing while still keeping project rules in the core library.

The generic [command-line/all](../../adapters/command-line/all.md) guide remains the real adapter setup path behind this choice.

## Adapter Rule

A Commander-based adapter should stay thin:

- define commands, options, and argument parsing in the adapter
- call the core library with the parsed values
- print or otherwise return the result from the core library

Do not move validation or service logic into Commander handlers if those rules belong in the core project logic.

## Ready State

The Commander setup is ready when:

- the adapter repo already follows the generic `command-line/all` setup
- Commander-specific parsing and command code delegates to the core library
- the framework slot can be recorded as `commander` in adapter repo names
