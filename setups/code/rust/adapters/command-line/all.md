<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../../../README.md) / [Setups](../../../../../README.md) / [Code](../../../../README.md) / [rust](../../../README.md) / [Adapters](../../README.md) / [Command Line](../README.md) / all.md
<!-- breadcrumbs:end -->

# All

Reusable Rust adapter setup for the `command-line/all` target.

## Goal

Create a thin command-line adapter that can call a well-tested core library without moving project rules into the entry point.

## Use This After

- [Toolchain](../../../toolchain.md)
- [Libtest](../../../testing/libtest.md)
- an existing core library repo for the project logic

## Scope

This guide covers:

- scaffolding a binary crate adapter repo with `cargo`
- adding a dependency on the core library from the adapter
- keeping the adapter thin

This guide does not cover:

- project-specific business rules
- command-line framework choices such as [Clap](../../../frameworks/command-line/clap.md) or [Ratatui](../../../frameworks/command-line/ratatui.md)
- project-specific command parsing

## Suggested Scaffold Commands

Replace the placeholders with the names used by the project tutorial:

```bash
cargo new <adapter-name> --bin <repo-root>
```

Then add the core library dependency in `Cargo.toml` using the mechanism that fits your workflow, such as a registry or git dependency.

## Suggested File Shape

```text
<repo-root>/
  Cargo.toml
  src/
    main.rs
```

The core library repo should stay separate. The adapter repo should depend on it as a real dependency rather than by copying the core source files.

## Adapter Rule

The `main.rs` entry point should stay thin:

- read input from the command line
- call the core library
- write the output

Do not duplicate validation or service rules in the adapter if those rules belong in the core project logic.

## Ready State

The Rust `command-line/all` adapter setup is ready when:

- the binary crate exists
- the adapter repo depends on the core library
- the binary entry point can delegate to the core library
- any optional command-line framework still delegates to the core library
