---
partial_kind: adapter-instructions
project: filtering-values
---

# Adapter Instructions

Project-specific adapter instruction fragment for the `filtering-values` adapter repo.

### 1. Red: Add An Adapter-Level Test

In a separate adapter repo, add a failing test that proves the chosen adapter parses the space-separated input into whole numbers, preserves the original order, and delegates even-number filtering and final formatting to the core logic correctly.

### 2. Green: Add The Thin Surface Adapter

Add the thinnest possible adapter for the surface you are building:

- render the equivalent of:
  - `Enter a list of numbers, separated by spaces:`
- collect one line of input
- split the line on spaces
- parse the resulting values into whole numbers
- preserve their original order
- pass the parsed list to `filter_even_numbers`
- pass the returned list to `format_even_numbers_report`
- render, return, or print the formatted result in the form that surface requires
- keep raw parsing, transport, and input/output code out of the core filtering logic

### 3. Refactor

Clean up any remaining duplication while keeping the full suite green.

