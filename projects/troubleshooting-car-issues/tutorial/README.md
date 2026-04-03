<!-- breadcrumbs:start -->
[for-all_tutorials](../../../README.md) / [Projects](../../README.md) / [Troubleshooting Car Issues](../README.md) / Tutorial
<!-- breadcrumbs:end -->

# Tutorial

Instructions for building `troubleshooting-car-issues` from the spec using one setup path and separate core and adapter walkthroughs.

## Contents

- [Core Walkthrough](core.md)
- [Adapter Walkthrough](adapter.md)

## Project-Specific Flow

Follow the shared repository-creation and tutorial workflow in [Projects](../../README.md#shared-repository-creation) and [Projects](../../README.md#shared-tutorial-workflow), then use this project's walkthrough files:

- core repo: [Core Walkthrough](core.md)
- adapter repo: [Adapter Walkthrough](adapter.md)

For this project, the core repo should expose `diagnose_car_issue` and `format_car_diagnosis_message`, and the adapter repo should ask only the needed branch prompts before delegating diagnosis resolution and message formatting to those core functions.

Use the shared finish checklist in [Projects](../../README.md#shared-finish-checklist) together with the project-specific rules in [Spec](../spec/README.md).
