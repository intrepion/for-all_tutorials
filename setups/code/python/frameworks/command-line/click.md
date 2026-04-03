<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../../README.md) / [Setups](../../../../README.md) / [Code](../../../README.md) / [python](../../README.md) / [Frameworks](../README.md) / [Command Line](README.md) / click.md
<!-- breadcrumbs:end -->

# Click

Reusable Python command-line framework guidance for adapters that use Click for commands, options, and argument parsing.

## Use This After

- [Toolchain](../../toolchain/README.md)
- [All](../../adapters/command-line/all.md)

## Guidance

Choose this option when the adapter should use Click for command definitions, options, and argument parsing while still keeping project rules in the core library.

The generic [command-line/all](../../adapters/command-line/all.md) guide remains the real adapter setup path behind this choice.

## Adapter Rule

A Click-based adapter should stay thin:

- define commands, options, and argument parsing in the adapter
- call the core library with the parsed values
- print or otherwise return the result from the core library

Do not move validation or service logic into Click handlers if those rules belong in the core project logic.

## Ready State

The Click setup is ready when:

- the adapter repo already follows the generic `command-line/all` setup
- Click-specific parsing and command code delegates to the core library
- the framework slot can be recorded as `click` in output repo names
