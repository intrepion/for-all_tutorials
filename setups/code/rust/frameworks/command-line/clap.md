<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../../README.md) / [Setups](../../../../README.md) / [Code](../../../README.md) / [rust](../../README.md) / [Frameworks](../README.md) / [Command Line](README.md) / clap.md
<!-- breadcrumbs:end -->

# Clap

Reusable Rust command-line framework guidance for adapters that use Clap for argument parsing and command structure.

## Use This After

- [Toolchain](../../toolchain.md)
- [Libtest](../../testing/libtest.md)
- [All](../../adapters/command-line/all.md)

## Scope

This guide covers:

- adding Clap to a Rust command-line adapter repo
- keeping argument parsing and command definitions inside the adapter
- mapping parsed adapter input cleanly to the core library contract

This guide does not cover:

- project-specific business rules
- project-specific argument design
- moving validation or service logic out of the core library

## Suggested Setup

Add Clap to the adapter repo:

```bash
cargo add clap
```

If the adapter uses Clap derive macros or subcommands, keep those concerns in the adapter crate and treat the core library as the owner of the project rules.

## Adapter Rule

A Clap-based adapter should stay thin:

- parse command-line input with Clap
- call the core library with the parsed values
- print or otherwise return the result from the core library

Do not move project rules into Clap parsers, command handlers, or argument types if those rules belong in the core project logic.

## Ready State

The Clap setup is ready when:

- the adapter repo already follows the generic `command-line/all` setup
- the adapter repo depends on `clap`
- Clap parsing code delegates to the core library
