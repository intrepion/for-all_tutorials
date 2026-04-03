<!-- breadcrumbs:start -->
[for-all_tutorials](../../../README.md) / [Projects](../../README.md) / [Password Generator](../README.md) / Tutorial
<!-- breadcrumbs:end -->

# Tutorial

Instructions for building `password-generator` from the spec using one setup path and separate core and adapter walkthroughs.

## Contents

- [Core Walkthrough](core.md)
- [Adapter Walkthrough](adapter.md)

## Project-Specific Flow

Follow the shared tutorial workflow in [Projects](../../README.md#shared-tutorial-workflow), then use this project's walkthrough files:

- core repo: [Core Walkthrough](core.md)
- adapter repo: [Adapter Walkthrough](adapter.md)

For this project, the core repo should expose `password_generation_plan`, `letter_character_pool`, `digit_character_pool`, `special_character_pool`, and `format_generated_password`, and the adapter repo should handle random character selection and shuffling while delegating the canonical generation rules and output formatting to those core functions.

Use the shared finish checklist in [Projects](../../README.md#shared-finish-checklist) together with the project-specific rules in [Spec](../spec/README.md).
