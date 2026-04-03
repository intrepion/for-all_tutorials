<!-- breadcrumbs:start -->
[for-all_tutorials](../../../README.md) / [Projects](../../README.md) / [Temperature Converter](../README.md) / Tutorial
<!-- breadcrumbs:end -->

# Tutorial

Instructions for building `temperature-converter` from the spec using one setup path and separate core and adapter walkthroughs.

## Contents

- [Core Walkthrough](core.md)
- [Adapter Walkthrough](adapter.md)

## Project-Specific Flow

Follow the shared tutorial workflow in [Projects](../../README.md#shared-tutorial-workflow), then use this project's walkthrough files:

- core repo: [Core Walkthrough](core.md)
- adapter repo: [Adapter Walkthrough](adapter.md)

For this project, the core repo should expose `convert_temperature_tenths` and `format_temperature_conversion_report`, and the adapter repo should parse the prompted temperature into tenths and pass the exact conversion choice expected by the core before delegating conversion and report formatting to those core functions.

Use the shared finish checklist in [Projects](../../README.md#shared-finish-checklist) together with the project-specific rules in [Spec](../spec/README.md).
