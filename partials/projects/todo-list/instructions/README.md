---
partial_kind: instructions-index
project: todo-list
---

# Instructions

Instructions for building `todo-list` from the spec using one setup path and separate core and adapter walkthroughs.

## Contents

- [Core Instructions](core.md)
- [Adapter Instructions](adapter.md)

## Project-Specific Flow

Follow the shared repository-creation and instruction workflow in [Projects](../../README.md#shared-repository-creation) and [Projects](../../README.md#shared-instruction-workflow), then use this project's instruction files:

- core repo: [Core Instructions](core.md)
- adapter repo: [Adapter Instructions](adapter.md)

For this project, the core repo should expose `parse_task_storage`, `append_task`, `remove_task_by_exact_text`, `format_task_list`, and `serialize_task_storage`, and the adapter repo should manage the chosen interaction flow and durable storage mechanism by delegating task parsing, list updates, formatting, and serialization to those core functions.

This file is a source index, not a compiled tutorial. Use the shared finish checklist in [Projects](../../README.md#shared-finish-checklist) together with the project-specific rules in [Spec](../spec/README.md) and the declared outputs in [Manifest](../manifest.yaml).
