<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../../README.md) / [Setups](../../../../README.md) / [Code](../../../README.md) / [php](../../README.md) / [Frameworks](../README.md) / [Command Line](README.md) / symfony-console.md
<!-- breadcrumbs:end -->

# Symfony Console

Reusable PHP command-line framework guidance for adapters that use Symfony Console for commands, options, and argument parsing.

## Use This After

- [Toolchain](../../toolchain/README.md)
- [All](../../adapters/command-line/all.md)

## Guidance

Choose this option when the adapter should use Symfony Console for command definitions, options, and argument parsing while still keeping project rules in the core library.

The generic [command-line/all](../../adapters/command-line/all.md) guide remains the real adapter setup path behind this choice.

## Adapter Rule

A Symfony Console adapter should stay thin:

- define commands, options, and argument parsing in the adapter
- call the core library with the parsed values
- print or otherwise return the result from the core library

Do not move validation or service logic into Symfony Console command handlers if those rules belong in the core project logic.

## Ready State

The Symfony Console setup is ready when:

- the adapter repo already follows the generic `command-line/all` setup
- Symfony Console-specific parsing and command code delegates to the core library
- the framework slot can be recorded as `symfony-console` in output repo names
