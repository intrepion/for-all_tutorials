<!-- breadcrumbs:start -->
[for-all_tutorials](../../../README.md) / [Projects](../../README.md) / [Counting the Number of Characters](../README.md) / Tutorial
<!-- breadcrumbs:end -->

# Tutorial

Instructions for building `count-characters` from the spec using one setup path and the TDD walkthrough.

## Contents

- [TDD Walkthrough](tdd.md)

## Project-Specific Flow

Follow the shared tutorial workflow in [Projects](../../README.md#shared-tutorial-workflow), then use this project's step ranges:

- core repo: [TDD Walkthrough](tdd.md) steps `1` through `21`
- adapter repo: [TDD Walkthrough](tdd.md) steps `22` through `24`

For this project, the core repo should expose `count_characters` and `format_character_count_message`, and the adapter repo should compose them without redefining their rules.

Use the shared finish checklist in [Projects](../../README.md#shared-finish-checklist) together with the project-specific rules in [Spec](../spec/README.md).
