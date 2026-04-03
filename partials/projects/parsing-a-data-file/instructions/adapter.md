---
partial_kind: adapter-instructions
project: parsing-a-data-file
---

# Adapter Instructions

Project-specific adapter instruction fragment for the `parsing-a-data-file` adapter repo.

### 1. Red: Add An Adapter-Level Test

In a separate adapter repo, add a failing test that proves the chosen adapter reads the canonical source file, delegates CSV parsing and table formatting to the core logic, and renders the returned lines in order.

### 2. Green: Add The Thin Surface Adapter

Add the thinnest possible adapter for the surface you are building:

- read the source file contents
- split the source file contents into lines
- pass those lines to `parse_employee_salary_records`
- pass the parsed records to `format_employee_salary_table`
- render the returned lines in order
- keep file input/output code out of the core data-parsing logic

### 3. Refactor

Clean up any remaining duplication while keeping the full suite green.

