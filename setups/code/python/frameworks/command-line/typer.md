<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../../README.md) / [Setups](../../../../README.md) / [Code](../../../README.md) / [python](../../README.md) / [Frameworks](../README.md) / [Command Line](README.md) / typer.md
<!-- breadcrumbs:end -->

# Typer

Reusable Python command-line framework guidance for adapters that use Typer for typed commands, options, and argument parsing.

## Use This After

- [Toolchain](../../toolchain/README.md)
- [All](../../adapters/command-line/all.md)

## Guidance

Choose this option when the adapter should use Typer for typed command definitions and argument parsing while still keeping project rules in the core library.

The generic [command-line/all](../../adapters/command-line/all.md) guide remains the real adapter setup path behind this choice.

## Adapter Rule

A Typer-based adapter should stay thin:

- define commands, options, and argument parsing in the adapter
- call the core library with the parsed values
- print or otherwise return the result from the core library

Do not move validation or service logic into Typer handlers if those rules belong in the core project logic.

## Ready State

The Typer setup is ready when:

- the adapter repo already follows the generic `command-line/all` setup
- Typer-specific parsing and command code delegates to the core library
- the framework slot can be recorded as `typer` in output repo names
