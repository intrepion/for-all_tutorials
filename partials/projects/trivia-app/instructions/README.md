---
partial_kind: instructions-index
project: trivia-app
---

# Instructions

Instructions for building `trivia-app` from the spec using one setup path and separate core and adapter walkthroughs.

## Contents

- [Core Instructions](core.md)
- [Adapter Instructions](adapter.md)

## Project-Specific Flow

Follow the shared repository-creation and instruction workflow in [Projects](../../README.md#shared-repository-creation) and [Projects](../../README.md#shared-instruction-workflow), then use this project's instruction files:

- core repo: [Core Instructions](core.md)
- adapter repo: [Adapter Instructions](adapter.md)

For this project, the core repo should expose `parse_trivia_question_bank`, `build_trivia_choice_pool`, `is_correct_trivia_answer`, and `format_trivia_final_score`, and the adapter repo should implement file reading, random question selection, random answer ordering, and game flow by delegating to those core functions.

This file is a source index, not a compiled tutorial. Use the shared finish checklist in [Projects](../../README.md#shared-finish-checklist) together with the project-specific rules in [Spec](../spec/README.md) and the declared outputs in [Manifest](../manifest.yaml).

