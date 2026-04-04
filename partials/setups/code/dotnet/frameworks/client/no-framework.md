---
partial_kind: framework-partial
ecosystem: dotnet
surface: client
framework: no-framework
---

# No Framework

Explicit no-framework choice for `.NET` `client` adapters.

## Use This After

- [Toolchain](../../toolchain/README.md)
- one guide from [Testing](../../testing/README.md)
- [All](../../adapters/client/all.md)

## Guidance

Choose this option when the client adapter should use the smallest built-in .NET app shape for the selected run without adding an extra client framework guide.

The generic [client/all](../../adapters/client/all.md) guide is the real setup path for this choice.

Use `no-framework` as the framework slot in adapter repo names when you want naming consistency across runs.

## Ready State

The `.NET` no-framework client choice is ready when the adapter repo follows the generic `client/all` guide without adding an extra client-framework layer.
