<!-- breadcrumbs:start -->
[for-all_tutorials](../../../README.md) / [Setups](../../README.md) / [C#](../README.md) / Testing
<!-- breadcrumbs:end -->

# Testing

Reusable unit-test setup guides for C# tutorials.

## Contents

- [xUnit](xunit/README.md)
- [NUnit](nunit/README.md)
- [MSTest](mstest/README.md)
- [TUnit](tunit/README.md)

## Guidance

Choose one testing framework for a single run of a tutorial and keep it consistent for that run.

In general:

- xUnit, NUnit, and MSTest commonly use `dotnet new` test-project templates
- TUnit often starts with either an installed template or a manual package-based setup

Project tutorials should link here instead of repeating test-runner setup in every project folder.
