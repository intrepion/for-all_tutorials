<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../README.md) / [Projects](../../../README.md) / [Simple Math](../../README.md) / [Tutorial](../README.md) / adapter.md
<!-- breadcrumbs:end -->

# Adapter Walkthrough

Project-specific red, green, refactor sequence for the `simple-math` adapter repo.

### 1. Red: Add An Adapter-Level Test

In a separate adapter repo, add a failing test that proves the chosen adapter delegates to the core logic correctly.

### 2. Green: Add The Thin Surface Adapter

Add the thinnest possible adapter for the surface you are building:

- collect one first-number input string from that surface
- collect one second-number input string from that surface
- parse those strings into numeric values that match the core contract
- pass the parsed numbers to `calculate_simple_math`
- pass the original numeric values plus the returned results to `format_simple_math_report`
- return, render, or print the four lines in the form that surface requires
- keep transport, parsing, and input/output code out of the core arithmetic logic

For prompt-driven adapters, the prompts should be equivalent to:

```text
What is the first number?
What is the second number?
```

### 3. Refactor

Clean up any remaining duplication while keeping the full suite green.
