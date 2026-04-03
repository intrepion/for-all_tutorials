<!-- breadcrumbs:start -->
[for-all_tutorials](../../../README.md) / [Projects](../../README.md) / [Todo List](../README.md) / Tutorial
<!-- breadcrumbs:end -->

# Tutorial

Instructions for building `todo-list` from the spec using one setup path and separate core and adapter walkthroughs.

## Contents

- [Core Walkthrough](core.md)
- [Adapter Walkthrough](adapter.md)

## Project-Specific Flow

Follow the shared tutorial workflow in [Projects](../../README.md#shared-tutorial-workflow), then use this project's walkthrough files:

- core repo: [Core Walkthrough](core.md)
- adapter repo: [Adapter Walkthrough](adapter.md)

For this project, the core repo should expose `parse_task_storage`, `append_task`, `remove_task_by_exact_text`, `format_task_list`, and `serialize_task_storage`, and the adapter repo should manage prompt flow and permanent storage by delegating task parsing, list updates, formatting, and serialization to those core functions.

Use the shared finish checklist in [Projects](../../README.md#shared-finish-checklist) together with the project-specific rules in [Spec](../spec/README.md).
