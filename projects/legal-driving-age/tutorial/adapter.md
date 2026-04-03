<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../README.md) / [Projects](../../../README.md) / [Legal Driving Age](../../README.md) / [Tutorial](../README.md) / adapter.md
<!-- breadcrumbs:end -->

# Adapter Walkthrough

Project-specific red, green, refactor sequence for the `legal-driving-age` adapter repo.

### 1. Red: Add An Adapter-Level Test

In a separate adapter repo, add a failing test that proves the chosen adapter delegates to the core logic correctly.

### 2. Green: Add The Thin Surface Adapter

Add the thinnest possible adapter for the surface you are building:

- collect one age input string from that surface
- parse the age into a whole number that matches the core contract
- pass the parsed age to `is_old_enough_to_drive`
- pass the returned boolean value to `format_driving_eligibility_message`
- return, render, or print the returned message in the form that surface requires
- keep transport, parsing, and input/output code out of the core comparison and message logic

For prompt-driven adapters, the prompts should be equivalent to:

```text
What is your age?
```

### 3. Refactor

Clean up any remaining duplication while keeping the full suite green.
