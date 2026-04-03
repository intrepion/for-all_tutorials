<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../../../README.md) / [Setups](../../../../../README.md) / [Code](../../../../README.md) / [javascript](../../../README.md) / [Adapters](../../README.md) / [Command Line](../README.md) / all.md
<!-- breadcrumbs:end -->

# All

Reusable JavaScript adapter setup for the `command-line/all` target.

## Goal

Create a thin command-line adapter that can read input, write output, and delegate project rules to a well-tested core library.

## Use This After

- [Toolchain](../../../toolchain/README.md)
- an existing core library repo for the project logic
- one guide from [Testing](../../../testing/README.md) when a JavaScript testing guide has been added

## Adapter Rule

The JavaScript `command-line/all` adapter should stay thin:

- read input from the command line
- call the core library
- write the output

## Ready State

The JavaScript `command-line/all` adapter setup is ready when:

- the adapter repo exists
- the adapter repo depends on the core library
- the command-line entry point can delegate to the core library
