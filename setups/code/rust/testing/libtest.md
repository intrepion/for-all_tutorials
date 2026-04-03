<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../README.md) / [Setups](../../../README.md) / [Code](../../README.md) / [rust](../README.md) / [Testing](README.md) / libtest.md
<!-- breadcrumbs:end -->

# Libtest

Reusable `libtest` setup for Rust tutorials.

## Goal

Use Rust's built-in `libtest` support with `cargo test` for spec-driven, test-driven tutorial runs.

## Use This After

- [Toolchain](../toolchain/README.md)

## Scope

This guide covers:

- using inline unit tests or integration tests with `cargo test`
- keeping the default Rust testing flow simple for small tutorial repos

This guide does not cover:

- third-party test frameworks
- property-based testing libraries
- project-specific assertions

## Suggested Starting Point

For a small core-library repo, start with tests close to the code they exercise:

```text
<repo-root>/
  Cargo.toml
  src/
    lib.rs
```

Then run:

```bash
cargo test --manifest-path <repo-root>/Cargo.toml
```

## Ready State

The Rust `libtest` setup is ready when:

- test code compiles
- `cargo test` runs successfully
- the tutorial can follow its red, green, refactor sequence without extra testing dependencies
