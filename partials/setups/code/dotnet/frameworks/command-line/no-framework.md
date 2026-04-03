---
partial_kind: framework-partial
ecosystem: dotnet
surface: command-line
framework: no-framework
---

# No Framework

Explicit no-framework choice for `.NET` command-line adapters.

## Use This After

- [Toolchain](../../toolchain/README.md)
- one guide from [Testing](../../testing/README.md)
- [All](../../adapters/command-line/all.md)

## Guidance

Choose this option when the adapter should use the base console APIs without adding a command-line library such as Spectre Console.

The generic [command-line/all](../../adapters/command-line/all.md) guide is the real setup path for this choice.

Use `no-framework` as the framework slot in adapter repo names when you want naming consistency across runs.

## Ready State

The `.NET` no-framework command-line choice is ready when the adapter repo follows the generic `command-line/all` guide without adding an extra command-line library.
