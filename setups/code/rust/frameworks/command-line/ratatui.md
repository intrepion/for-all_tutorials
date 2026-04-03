<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../../README.md) / [Setups](../../../../README.md) / [Code](../../../README.md) / [rust](../../README.md) / [Frameworks](../README.md) / [Command Line](README.md) / ratatui.md
<!-- breadcrumbs:end -->

# Ratatui

Reusable Rust command-line framework guidance for adapters that use Ratatui for richer terminal user interfaces.

## Use This After

- [Toolchain](../../toolchain/README.md)
- [Libtest](../../testing/libtest.md)
- [All](../../adapters/command-line/all.md)

## Scope

This guide covers:

- adding Ratatui to a Rust command-line adapter repo
- keeping terminal layout, rendering, and event-loop concerns inside the adapter
- mapping terminal interactions cleanly to the core library contract

This guide does not cover:

- project-specific business rules
- choosing a full terminal backend strategy
- moving validation or service logic out of the core library

## Suggested Setup

Add Ratatui to the adapter repo:

```bash
cargo add ratatui
```

If the adapter grows a terminal event loop, stateful view model, or richer layout system, keep those concerns in the adapter crate and continue treating the core library as the owner of the project rules.

## Adapter Rule

A Ratatui-based adapter should stay thin:

- gather terminal events and UI state in the adapter
- call the core library for the project behavior
- render the result back through the terminal UI

Do not move project rules into the terminal loop, view-state transitions, or rendering layer if those rules belong in the core project logic.

## Ready State

The Ratatui setup is ready when:

- the adapter repo already follows the generic `command-line/all` setup
- the adapter repo depends on `ratatui`
- Ratatui-specific terminal logic delegates to the core library
