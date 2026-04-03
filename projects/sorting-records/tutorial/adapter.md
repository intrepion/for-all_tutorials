<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../README.md) / [Projects](../../../README.md) / [Sorting Records](../../README.md) / [Tutorial](../README.md) / adapter.md
<!-- breadcrumbs:end -->

# Adapter Walkthrough

Project-specific red, green, refactor sequence for the `sorting-records` adapter repo.

### 1. Red: Add An Adapter-Level Test

In a separate adapter repo, add a failing test that proves the chosen adapter obtains the canonical employee dataset, delegates sorting and table formatting to the core logic, and renders the returned table lines in order.

### 2. Green: Add The Thin Surface Adapter

Add the thinnest possible adapter for the surface you are building:

- call `default_employee_records`
- pass the returned dataset to `sort_employee_records_by_last_name`
- pass the sorted records to `format_employee_record_table`
- render the returned lines in order
- keep transport and input/output code out of the core record-sorting logic

### 3. Refactor

Clean up any remaining duplication while keeping the full suite green.
