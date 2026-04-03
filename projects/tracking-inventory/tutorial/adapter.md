<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../README.md) / [Projects](../../../README.md) / [Tracking Inventory](../../README.md) / [Tutorial](../README.md) / adapter.md
<!-- breadcrumbs:end -->

# Adapter Walkthrough

Project-specific red, green, refactor sequence for the `tracking-inventory` adapter repo.

### 1. Red: Add An Adapter-Level Test

In a separate adapter repo, add a failing test that proves the chosen adapter collects inventory item input, validates numeric value input, persists records in one local structured data file, reloads those records through the core parser, and renders both the HTML and CSV reports correctly.

### 2. Green: Add The Thin Adapter

Add the thinnest possible adapter for the surface you are building:

- collect `name`, `serial number`, and `estimated value` for each item
- validate the entered value with `validate_inventory_value`
- reject non-numeric value input
- pass accepted values to `build_inventory_record`
- persist the resulting records in one local structured data file using JSON, XML, or YAML
- read the same file back into text
- pass that stored text to `parse_inventory_storage`
- pass the parsed records to `format_inventory_html_report`
- pass the parsed records to `format_inventory_csv_report`
- render or write both report formats as the surface requires
- keep prompt flow and file I/O code out of the core validation and export logic

### 3. Refactor

Clean up any remaining duplication while keeping the full suite green.
