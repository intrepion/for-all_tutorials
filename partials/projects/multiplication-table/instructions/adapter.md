---
partial_kind: adapter-instructions
project: multiplication-table
---

# Adapter Instructions

Project-specific adapter instruction fragment for the `multiplication-table` adapter repo.

### 1. Red: Add An Adapter-Level Test

In a separate adapter repo, add a failing test that proves the chosen adapter delegates to the core logic correctly.

### 2. Green: Add The Thin Surface Adapter

Add the thinnest possible adapter for the surface you are building:

- call `generate_multiplication_table`
- pass the generated rows to `format_multiplication_table`
- return, render, or print the formatted lines in order
- keep transport and output code out of the core table-generation logic

This project has no user prompts. The adapter only renders the complete table.

### 3. Refactor

Clean up any remaining duplication while keeping the full suite green.

