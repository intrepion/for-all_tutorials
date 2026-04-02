<!-- breadcrumbs:start -->
[for-all_tutorials](../../../README.md) / [Projects](../../README.md) / [Saying Hello](../README.md) / Tutorial
<!-- breadcrumbs:end -->

# Tutorial

Instructions for building `saying-hello` from the spec using one setup path and the TDD walkthrough.

## Contents

- [TDD Walkthrough](tdd.md)

## Project-Specific Flow

Follow the shared tutorial workflow in [Projects](../../README.md#shared-tutorial-workflow), then use this project's step ranges:

- core repo: [TDD Walkthrough](tdd.md) steps `1` through `12`
- adapter repo: [TDD Walkthrough](tdd.md) steps `13` through `15`

For this project, the core repo should expose `greet`, and the adapter repo should delegate to it without redefining greeting rules.

Use the shared finish checklist in [Projects](../../README.md#shared-finish-checklist) together with the project-specific rules in [Spec](../spec/README.md).
