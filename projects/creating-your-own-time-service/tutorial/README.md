<!-- breadcrumbs:start -->
[for-all_tutorials](../../../README.md) / [Projects](../../README.md) / [Creating Your Own Time Service](../README.md) / Tutorial
<!-- breadcrumbs:end -->

# Tutorial

Instructions for building `creating-your-own-time-service` from the spec using one setup path and separate core and adapter walkthroughs.

## Contents

- [Core Walkthrough](core.md)
- [Adapter Walkthrough](adapter.md)

## Project-Specific Flow

Follow the shared tutorial workflow in [Projects](../../README.md#shared-tutorial-workflow), then use this project's walkthrough files:

- core repo: [Core Walkthrough](core.md)
- adapter repos: [Adapter Walkthrough](adapter.md)

For this project, the core repo should expose `build_current_time_payload`, `format_current_time_service_response`, `parse_current_time_service_response`, and `format_current_time_display`. A complete run should then build both a service adapter repo and a client adapter repo that delegate to those core functions.

Use the shared finish checklist in [Projects](../../README.md#shared-finish-checklist) together with the project-specific rules in [Spec](../spec/README.md).
