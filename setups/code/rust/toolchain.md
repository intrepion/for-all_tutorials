<!-- breadcrumbs:start -->
[for-all_tutorials](../../../README.md) / [Setups](../../README.md) / [Code](../README.md) / [rust](README.md) / toolchain.md
<!-- breadcrumbs:end -->

# Toolchain

Core Rust toolchain guidance for project tutorials that use the Rust ecosystem.

## Goal

Set up a clean, repeatable foundation for Rust output repos without tying the instructions to any one local machine.

## Scope

This guide covers:

- the base `cargo` CLI workflow
- library-crate scaffolding for core-library repos
- common repository conventions for Rust tutorials

This guide does not cover:

- adapter-specific scaffolding
- framework-specific bootstrapping
- project-specific business logic

## Recommended Conventions

- keep source in `src/`
- keep the core repo focused on one small library crate when the tutorial stays small
- keep formatting and linting consistent across runs when the chosen workflow supports it

## Verify The Toolchain

Before starting a Rust tutorial, confirm that the Rust toolchain is available:

```bash
rustc --version
cargo --version
```

## Suggested Scaffold Commands

For a core-library repo, use placeholders and substitute the values that fit the project you are building:

```bash
cargo new <crate-name> --lib <repo-root>
cargo test --manifest-path <repo-root>/Cargo.toml
```

## Suggested File Shape

```text
<repo-root>/
  Cargo.toml
  src/
    lib.rs
```

If the generated crate uses placeholder names or examples that do not match the project tutorial, replace them with the production names used by that tutorial.

For an adapter repo, use this guide for the base `cargo` conventions, then switch to the relevant adapter setup guide for the actual binary crate scaffold.

## Ready State

For a core-library repo, the base Rust toolchain setup is ready when:

- `rustc` and `cargo` commands run successfully
- the library crate exists
- the generated crate can run tests through `cargo test`
