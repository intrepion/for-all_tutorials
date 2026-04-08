---
partial_kind: instructions-index
project: saying-hello
---

# Instructions

Source index for the `saying-hello` instruction partials used to assemble future compiled tutorials.

## Contents

- [Contracts Instructions](contracts.md)
- [Core Instructions for xUnit](core.md)
- [Core Instructions for NUnit](core-nunit.md)
- [Core Instructions for MSTest](core-mstest.md)
- [Core Instructions for TUnit](core-tunit.md)
- [Adapter Instructions for xUnit](adapter-xunit.md)
- [Adapter Instructions for NUnit](adapter-nunit.md)
- [Adapter Instructions for MSTest](adapter-mstest.md)
- [Adapter Instructions for TUnit](adapter-tunit.md)

## Project-Specific Flow

Follow the shared repository-creation and instruction workflow in [Projects](../../README.md#shared-repository-creation) and [Projects](../../README.md#shared-instruction-workflow), then use this project's instruction files:

- contracts repo: [Contracts Instructions](contracts.md)
- core repo:
  - [Core Instructions for xUnit](core.md)
  - [Core Instructions for NUnit](core-nunit.md)
  - [Core Instructions for MSTest](core-mstest.md)
  - [Core Instructions for TUnit](core-tunit.md)
- adapter repo:
  - [Adapter Instructions for xUnit](adapter-xunit.md)
  - [Adapter Instructions for NUnit](adapter-nunit.md)
  - [Adapter Instructions for MSTest](adapter-mstest.md)
  - [Adapter Instructions for TUnit](adapter-tunit.md)

For this project, the contracts repo should define `IGreetingService`, the core repo should implement that contract, and the adapter repo should depend on the contract without redefining greeting rules.

This file is a source index, not a compiled tutorial. Use the shared finish checklist in [Projects](../../README.md#shared-finish-checklist) together with the project-specific rules in [Spec](../spec/README.md) and the declared outputs in [Manifest](../manifest.yaml).
