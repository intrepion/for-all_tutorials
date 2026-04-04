<!-- breadcrumbs:start -->
[for-all_tutorials](../README.md) / Setups
<!-- breadcrumbs:end -->

# Setups

Reusable ecosystem, tooling, testing, and storage setup guides that project tutorials can reference instead of repeating environment-specific instructions.

## How To Navigate

1. Start in `code/` or `storage/`, depending on the kind of setup you need.
2. Pick the ecosystem, such as `dotnet` or `rust`.
3. Use `languages/` for language-specific guides inside that ecosystem.
4. Use `toolchain/` for SDKs, runtimes, CLIs, package managers, and similar base tools.
5. Use the shared `storage/` taxonomy to choose the storage slot when the adapter persists data.
6. Use ecosystem `storage/` guides for stack-specific persistence setup when they exist.
7. Use `testing/` for the test framework you want to use.
8. Use `adapters/` for the surface and target you are building, such as `command-line/all` or `web/full-stack`.
9. Use `frameworks/` for optional framework-specific setup layered on top of the chosen adapter.

## Contents

- [Code](code/README.md)
- [Storage](storage/README.md)
