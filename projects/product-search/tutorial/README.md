<!-- breadcrumbs:start -->
[for-all_tutorials](../../../README.md) / [Projects](../../README.md) / [Product Search](../README.md) / Tutorial
<!-- breadcrumbs:end -->

# Tutorial

Instructions for building `product-search` from the spec using one setup path and separate core and adapter walkthroughs.

## Contents

- [Core Walkthrough](core.md)
- [Adapter Walkthrough](adapter.md)

## Project-Specific Flow

Follow the shared repository-creation and tutorial workflow in [Projects](../../README.md#shared-repository-creation) and [Projects](../../README.md#shared-tutorial-workflow), then use this project's walkthrough files:

- core repo: [Core Walkthrough](core.md)
- adapter repo: [Adapter Walkthrough](adapter.md)

For this project, the core repo should expose `parse_product_catalog`, `find_product_by_exact_name`, `format_product_report`, and `format_product_not_found_message`, and the adapter repo should read the canonical inventory JSON, retry until a product is found, and render the canonical product report by delegating parsing, lookup, and formatting to those core functions.

Use the shared finish checklist in [Projects](../../README.md#shared-finish-checklist) together with the project-specific rules in [Spec](../spec/README.md).
