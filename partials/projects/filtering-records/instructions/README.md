---
partial_kind: instructions-index
project: filtering-records
---

# Instructions

Instructions for building `filtering-records` from the spec using one setup path and separate core and adapter walkthroughs.

## Contents

- [Core Instructions](core.md)
- [Adapter Instructions](adapter.md)

## Project-Specific Flow

Follow the shared repository-creation and instruction workflow in [Projects](../../README.md#shared-repository-creation) and [Projects](../../README.md#shared-instruction-workflow), then use this project's instruction files:

- core repo: [Core Instructions](core.md)
- adapter repo: [Adapter Instructions](adapter.md)

For this project, the core repo should expose `default_employee_records`, `filter_employee_records_by_search_string`, `sort_employee_records_by_last_name`, and `format_employee_record_table`, and the adapter repo should render the canonical filtered employee table by delegating dataset construction, filtering, sorting, and formatting to those core functions.

This file is a source index, not a compiled tutorial. Use the shared finish checklist in [Projects](../../README.md#shared-finish-checklist) together with the project-specific rules in [Spec](../spec/README.md) and the declared outputs in [Manifest](../manifest.yaml).

