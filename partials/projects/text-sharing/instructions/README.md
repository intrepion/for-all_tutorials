---
partial_kind: instructions-index
project: text-sharing
---

# Instructions

Instructions for building `text-sharing` from the spec using one setup path and separate core and adapter walkthroughs.

## Contents

- [Core Instructions](core.md)
- [Adapter Instructions](adapter.md)

## Project-Specific Flow

Follow the shared repository-creation and instruction workflow in [Projects](../../README.md#shared-repository-creation) and [Projects](../../README.md#shared-instruction-workflow), then use this project's instruction files:

- core repo: [Core Instructions](core.md)
- adapter repo: [Adapter Instructions](adapter.md)

For this project, the core repo should expose `build_text_snippet_record`, `format_text_snippet_path`, `format_text_snippet_edit_path`, `format_text_snippet_view`, and `build_text_snippet_edit_seed`, and the adapter repo should implement form handling, persistence, the share route, and the edit route by delegating to those core functions.

This file is a source index, not a compiled tutorial. Use the shared finish checklist in [Projects](../../README.md#shared-finish-checklist) together with the project-specific rules in [Spec](../spec/README.md) and the declared outputs in [Manifest](../manifest.yaml).

