---
partial_kind: adapter-instructions
project: name-sorter
---

# Adapter Instructions

Project-specific adapter instruction fragment for the `name-sorter` adapter repo.

### 1. Red: Add An Adapter-Level Test

In a separate adapter repo, add a failing test that proves the chosen adapter reads the canonical source file, delegates parsing, sorting, and report formatting to the core logic, and writes the returned lines to the output file in order.

### 2. Green: Add The Thin Surface Adapter

Add the thinnest possible adapter for the surface you are building:

- read the source file contents
- split the source file contents into lines
- pass those lines to `parse_name_entries`
- pass the parsed entries to `sort_name_entries`
- pass the sorted entries to `format_sorted_name_report`
- write the returned lines to the output file in order
- keep file input/output code out of the core name-sorting logic

### 3. Refactor

Clean up any remaining duplication while keeping the full suite green.

