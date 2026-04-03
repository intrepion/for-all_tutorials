<!-- breadcrumbs:start -->
[for-all_tutorials](../../../README.md) / [Projects](../../README.md) / [Handling Bad Input](../README.md) / Tutorial
<!-- breadcrumbs:end -->

# Tutorial

Instructions for building `handling-bad-input` from the spec using one setup path and separate core and adapter walkthroughs.

## Contents

- [Core Walkthrough](core.md)
- [Adapter Walkthrough](adapter.md)

## Project-Specific Flow

Follow the shared tutorial workflow in [Projects](../../README.md#shared-tutorial-workflow), then use this project's walkthrough files:

- core repo: [Core Walkthrough](core.md)
- adapter repo: [Adapter Walkthrough](adapter.md)

For this project, the core repo should expose `parse_rate_of_return`, `calculate_years_to_double`, `format_invalid_rate_message`, and `format_rule_of_72_report`, and the adapter repo should keep prompting until it gets a valid rate before delegating the calculation and final message creation to those core functions.

Use the shared finish checklist in [Projects](../../README.md#shared-finish-checklist) together with the project-specific rules in [Spec](../spec/README.md).
