---
partial_kind: instructions-index
project: blood-alcohol-calculator
---

# Instructions

Instructions for building `blood-alcohol-calculator` from the spec using one setup path and separate core and adapter walkthroughs.

## Contents

- [Core Instructions](core.md)
- [Adapter Instructions](adapter.md)

## Project-Specific Flow

Follow the shared repository-creation and instruction workflow in [Projects](../../README.md#shared-repository-creation) and [Projects](../../README.md#shared-instruction-workflow), then use this project's instruction files:

- core repo: [Core Instructions](core.md)
- adapter repo: [Adapter Instructions](adapter.md)

For this project, the core repo should expose `calculate_bac_hundredths`, `is_legal_to_drive_with_bac`, and `format_bac_report`, and the adapter repo should parse the prompted inputs into the exact numeric and categorical values expected by the core before delegating BAC calculation, legal determination, and report formatting to those core functions.

This file is a source index, not a compiled tutorial. Use the shared finish checklist in [Projects](../../README.md#shared-finish-checklist) together with the project-specific rules in [Spec](../spec/README.md) and the declared outputs in [Manifest](../manifest.yaml).

