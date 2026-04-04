---
partial_kind: testing-index
ecosystem: dotnet
---

# Testing

Reusable unit-test setup partials for `.NET` compiled tutorials.

## Contents

- [xUnit](xunit.md)
- [NUnit](nunit.md)
- [MSTest](mstest.md)
- [TUnit](tunit.md)

## Guidance

Choose one testing framework for a single compiled tutorial output and keep it consistent for that run.

Testing-framework choice should affect:

- test-project scaffolding
- assertion syntax
- coverage workflow

It should not affect:

- the project contract
- the TDD order declared by the project instructions
- the behavioral requirements of the project
