<!-- breadcrumbs:start -->
[for-all_tutorials](../../../README.md) / [Projects](../../README.md) / [Legal Driving Age](../README.md) / Tutorial
<!-- breadcrumbs:end -->

# Tutorial

Instructions for building `legal-driving-age` from the spec using one setup path and separate core and adapter walkthroughs.

## Contents

- [Core Walkthrough](core.md)
- [Adapter Walkthrough](adapter.md)

## Project-Specific Flow

Follow the shared tutorial workflow in [Projects](../../README.md#shared-tutorial-workflow), then use this project's walkthrough files:

- core repo: [Core Walkthrough](core.md)
- adapter repo: [Adapter Walkthrough](adapter.md)

For this project, the core repo should expose `is_old_enough_to_drive` and `format_driving_eligibility_message`, and the adapter repo should parse the prompted age into a whole number before delegating eligibility comparison and message formatting to those core functions.

Use the shared finish checklist in [Projects](../../README.md#shared-finish-checklist) together with the project-specific rules in [Spec](../spec/README.md).
