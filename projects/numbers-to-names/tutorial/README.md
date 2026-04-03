<!-- breadcrumbs:start -->
[for-all_tutorials](../../../README.md) / [Projects](../../README.md) / [Numbers to Names](../README.md) / Tutorial
<!-- breadcrumbs:end -->

# Tutorial

Instructions for building `numbers-to-names` from the spec using one setup path and separate core and adapter walkthroughs.

## Contents

- [Core Walkthrough](core.md)
- [Adapter Walkthrough](adapter.md)

## Project-Specific Flow

Follow the shared tutorial workflow in [Projects](../../README.md#shared-tutorial-workflow), then use this project's walkthrough files:

- core repo: [Core Walkthrough](core.md)
- adapter repo: [Adapter Walkthrough](adapter.md)

For this project, the core repo should expose `lookup_month_name` and `format_month_lookup_message`, and the adapter repo should parse the prompted month number into a whole number before delegating month lookup and message formatting to those core functions.

Use the shared finish checklist in [Projects](../../README.md#shared-finish-checklist) together with the project-specific rules in [Spec](../spec/README.md).
