<!-- breadcrumbs:start -->
[for-all_tutorials](../../../README.md) / [Projects](../../README.md) / [Computing Statistics](../README.md) / Tutorial
<!-- breadcrumbs:end -->

# Tutorial

Instructions for building `computing-statistics` from the spec using one setup path and separate core and adapter walkthroughs.

## Contents

- [Core Walkthrough](core.md)
- [Adapter Walkthrough](adapter.md)

## Project-Specific Flow

Follow the shared tutorial workflow in [Projects](../../README.md#shared-tutorial-workflow), then use this project's walkthrough files:

- core repo: [Core Walkthrough](core.md)
- adapter repo: [Adapter Walkthrough](adapter.md)

For this project, the core repo should expose `mean_hundredths`, `minimum_response_time`, `maximum_response_time`, `standard_deviation_hundredths`, and `format_statistics_report`, and the adapter repo should collect response times until the sentinel value while delegating the statistical calculations and final report formatting to those core functions.

Use the shared finish checklist in [Projects](../../README.md#shared-finish-checklist) together with the project-specific rules in [Spec](../spec/README.md).
