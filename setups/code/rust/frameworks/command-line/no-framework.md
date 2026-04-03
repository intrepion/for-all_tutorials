<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../../README.md) / [Setups](../../../../README.md) / [Code](../../../README.md) / [rust](../../README.md) / [Frameworks](../README.md) / [Command Line](README.md) / no-framework.md
<!-- breadcrumbs:end -->

# No Framework

Explicit no-framework choice for Rust command-line adapters.

## Use This After

- [Toolchain](../../toolchain/README.md)
- [Libtest](../../testing/libtest.md)
- [All](../../adapters/command-line/all.md)

## Guidance

Choose this option when the adapter should use the standard library and base Rust binary-crate flow without adding a command-line library such as Clap or Ratatui.

The generic [command-line/all](../../adapters/command-line/all.md) guide is the real setup path for this choice.

Use `no-framework` as the framework slot in adapter repo names when you want naming consistency across runs.

## Ready State

The Rust no-framework command-line choice is ready when the adapter repo follows the generic `command-line/all` guide without adding an extra command-line library.
