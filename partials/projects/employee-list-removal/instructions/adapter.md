---
partial_kind: adapter-instructions
project: employee-list-removal
---

# Adapter Instructions

Project-specific adapter instruction fragment for the `employee-list-removal` adapter repo.

### 1. Red: Add An Adapter-Level Test

In a separate adapter repo, add a failing test that proves the chosen adapter renders the starting employee list, prompts once for the removal target, and delegates exact-match removal and final list rendering to the core logic correctly.

### 2. Green: Add The Thin Surface Adapter

Add the thinnest possible adapter for the surface you are building:

- call `default_employee_names`
- pass the returned list to `format_employee_list`
- render the formatted starting list
- render the equivalent of:
  - `Enter an employee name to remove:`
- collect the raw removal target
- pass the original canonical list and the raw removal target to `remove_employee_by_exact_name`
- pass the returned list to `format_employee_list`
- render the resulting formatted list in the form that surface requires
- keep transport and input/output code out of the core employee-list logic

### 3. Refactor

Clean up any remaining duplication while keeping the full suite green.

