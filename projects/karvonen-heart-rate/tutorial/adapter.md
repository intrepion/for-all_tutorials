<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../README.md) / [Projects](../../../README.md) / [Karvonen Heart Rate](../../README.md) / [Tutorial](../README.md) / adapter.md
<!-- breadcrumbs:end -->

# Adapter Walkthrough

Project-specific red, green, refactor sequence for the `karvonen-heart-rate` adapter repo.

### 1. Red: Add An Adapter-Level Test

In a separate adapter repo, add a failing test that proves the chosen adapter delegates to the core logic correctly.

### 2. Green: Add The Thin Surface Adapter

Add the thinnest possible adapter for the surface you are building:

- collect one resting-pulse value from that surface
- collect one age value from that surface
- parse both values into whole numbers
- pass the parsed values to `generate_karvonen_table`
- pass the original parsed inputs and generated rows to `format_karvonen_table`
- return, render, or print the formatted lines in order
- keep transport, parsing, and input/output code out of the core Karvonen logic

For prompt-driven adapters, the prompts should be equivalent to:

```text
Resting Pulse:
Age:
```

### 3. Refactor

Clean up any remaining duplication while keeping the full suite green.
