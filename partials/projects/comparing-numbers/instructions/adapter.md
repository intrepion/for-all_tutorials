---
partial_kind: adapter-instructions
project: comparing-numbers
---

# Adapter Instructions

Project-specific adapter instruction fragment for the `comparing-numbers` adapter repo.

### 1. Red: Add An Adapter-Level Test

In a separate adapter repo, add a failing test that proves the chosen adapter checks whether the three inputs are all different before selecting the largest number and that it stops cleanly when duplicate values are provided.

### 2. Green: Add The Thin Surface Adapter

Add the thinnest possible adapter for the surface you are building:

- render the equivalent of:
  - `Enter the first number:`
  - `Enter the second number:`
  - `Enter the third number:`
- parse the three prompted values into whole numbers
- pass those parsed values to `all_numbers_are_different`
- when the result is `false`, stop the current run without calling `largest_of_three` and without rendering `format_largest_number_message`
- when the result is `true`, pass the same three numbers to `largest_of_three`
- pass the resulting value to `format_largest_number_message`
- render, return, or print the formatted result in the form that surface requires
- keep parsing, transport, and input/output code out of the core comparison logic

### 3. Refactor

Clean up any remaining duplication while keeping the full suite green.

