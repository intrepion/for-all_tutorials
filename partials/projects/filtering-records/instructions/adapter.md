---
partial_kind: adapter-instructions
project: filtering-records
---

# Adapter Instructions

Project-specific adapter instruction fragment for the `filtering-records` adapter repo.

### 1. Red: Add An Adapter-Level Test

In a separate adapter repo, add a failing test that proves the chosen adapter obtains the canonical employee dataset, prompts for the search string, delegates filtering, sorting, and table formatting to the core logic, renders the `Results:` heading, and renders the returned table lines in order.

### 2. Green: Add The Thin Surface Adapter

Add the thinnest possible adapter for the surface you are building:

- call `default_employee_records`
- render the equivalent of `Enter a search string:`
- pass the canonical dataset and the entered search string to `filter_employee_records_by_search_string`
- pass the filtered records to `sort_employee_records_by_last_name`
- render the equivalent of `Results:`
- pass the sorted filtered records to `format_employee_record_table`
- render the returned lines in order
- keep transport and input/output code out of the core record-filtering logic

### 3. Refactor

Clean up any remaining duplication while keeping the full suite green.

