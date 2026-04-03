<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../README.md) / [Projects](../../../README.md) / [Adding Numbers](../../README.md) / [Tutorial](../README.md) / adapter.md
<!-- breadcrumbs:end -->

# Adapter Walkthrough

Project-specific red, green, refactor sequence for the `adding-numbers` adapter repo.

### 1. Red: Add An Adapter-Level Test

In a separate adapter repo, add a failing test that proves the chosen adapter delegates to the core logic correctly.

### 2. Green: Add The Thin Surface Adapter

Add the thinnest possible adapter for the surface you are building:

- prompt for one number exactly five times
- parse each prompted value into a whole number
- preserve the five-number input order
- pass the resulting list to `sum_numbers`
- pass the resulting total to `format_total_report`
- return, render, or print the formatted result in the form that surface requires
- keep transport, parsing, and input/output code out of the core summing logic

For prompt-driven adapters, each prompt should be equivalent to:

```text
Enter a number:
```

### 3. Refactor

Clean up any remaining duplication while keeping the full suite green.
