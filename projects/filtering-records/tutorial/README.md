<!-- breadcrumbs:start -->
[for-all_tutorials](../../../README.md) / [Projects](../../README.md) / [Filtering Records](../README.md) / Tutorial
<!-- breadcrumbs:end -->

# Tutorial

Instructions for building `filtering-records` from the spec using one setup path and separate core and adapter walkthroughs.

## Contents

- [Core Walkthrough](core.md)
- [Adapter Walkthrough](adapter.md)

## Project-Specific Flow

Follow the shared repository-creation and tutorial workflow in [Projects](../../README.md#shared-repository-creation) and [Projects](../../README.md#shared-tutorial-workflow), then use this project's walkthrough files:

- core repo: [Core Walkthrough](core.md)
- adapter repo: [Adapter Walkthrough](adapter.md)

For this project, the core repo should expose `default_employee_records`, `filter_employee_records_by_search_string`, `sort_employee_records_by_last_name`, and `format_employee_record_table`, and the adapter repo should render the canonical filtered employee table by delegating dataset construction, filtering, sorting, and formatting to those core functions.

Use the shared finish checklist in [Projects](../../README.md#shared-finish-checklist) together with the project-specific rules in [Spec](../spec/README.md).
