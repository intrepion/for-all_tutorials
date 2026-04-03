<!-- breadcrumbs:start -->
[for-all_tutorials](../../../README.md) / [Projects](../../README.md) / [Tracking Inventory](../README.md) / Tutorial
<!-- breadcrumbs:end -->

# Tutorial

Instructions for building `tracking-inventory` from the spec using one setup path and separate core and adapter walkthroughs.

## Contents

- [Core Walkthrough](core.md)
- [Adapter Walkthrough](adapter.md)

## Project-Specific Flow

Follow the shared tutorial workflow in [Projects](../../README.md#shared-tutorial-workflow), then use this project's walkthrough files:

- core repo: [Core Walkthrough](core.md)
- adapter repo: [Adapter Walkthrough](adapter.md)

For this project, the core repo should expose `validate_inventory_value`, `build_inventory_record`, `parse_inventory_storage`, `format_inventory_html_report`, and `format_inventory_csv_report`, and the adapter repo should implement prompt flow, persistent storage, and export generation by delegating to those core functions.

Use the shared finish checklist in [Projects](../../README.md#shared-finish-checklist) together with the project-specific rules in [Spec](../spec/README.md).
