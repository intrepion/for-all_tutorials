---
partial_kind: instructions-index
project: handling-bad-input
---

# Instructions

Instructions for building `handling-bad-input` from the spec using one setup path and separate core and adapter walkthroughs.

## Contents

- [Core Instructions](core.md)
- [Adapter Instructions](adapter.md)

## Project-Specific Flow

Follow the shared repository-creation and instruction workflow in [Projects](../../README.md#shared-repository-creation) and [Projects](../../README.md#shared-instruction-workflow), then use this project's instruction files:

- core repo: [Core Instructions](core.md)
- adapter repo: [Adapter Instructions](adapter.md)

For this project, the core repo should expose `parse_rate_of_return`, `calculate_years_to_double`, `format_invalid_rate_message`, and `format_rule_of_72_report`, and the adapter repo should keep prompting until it gets a valid rate before delegating the calculation and final message creation to those core functions.

This file is a source index, not a compiled tutorial. Use the shared finish checklist in [Projects](../../README.md#shared-finish-checklist) together with the project-specific rules in [Spec](../spec/README.md) and the declared outputs in [Manifest](../manifest.yaml).

