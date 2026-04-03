<!-- breadcrumbs:start -->
[for-all_tutorials](../../../README.md) / [Projects](../../README.md) / [Pushing Notes to Firebase](../README.md) / Tutorial
<!-- breadcrumbs:end -->

# Tutorial

Instructions for building `pushing-notes-to-firebase` from the spec using one setup path and separate core and adapter walkthroughs.

## Contents

- [Core Walkthrough](core.md)
- [Adapter Walkthrough](adapter.md)

## Project-Specific Flow

Follow the shared repository-creation and tutorial workflow in [Projects](../../README.md#shared-repository-creation) and [Projects](../../README.md#shared-tutorial-workflow), then use this project's walkthrough files:

- core repo: [Core Walkthrough](core.md)
- adapter repo: [Adapter Walkthrough](adapter.md)

For this project, the core repo should expose `build_note_record`, `parse_notes_collection_response`, `format_note_saved_message`, and `format_notes_report`, and the adapter repo should implement the `new` and `show` command behaviors by delegating note construction, parsing, and formatting to those core functions.

Use the shared finish checklist in [Projects](../../README.md#shared-finish-checklist) together with the project-specific rules in [Spec](../spec/README.md).
