<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../README.md) / [Projects](../../../README.md) / [Tax Calculator](../../README.md) / [Tutorial](../README.md) / adapter.md
<!-- breadcrumbs:end -->

# Adapter Walkthrough

Project-specific red, green, refactor sequence for the `tax-calculator` adapter repo.

### 1. Red: Add An Adapter-Level Test

In a separate adapter repo, add a failing test that proves the chosen adapter delegates to the core logic correctly.

### 2. Green: Add The Thin Surface Adapter

Add the thinnest possible adapter for the surface you are building:

- collect one order-amount input string from that surface
- collect one state input string from that surface
- parse the order amount into exact cents that match the core contract
- pass the parsed order amount and state to `calculate_tax_for_order`
- pass the returned summary to `format_tax_report`
- return, render, or print the returned lines in the form that surface requires
- keep transport, parsing, and input/output code out of the core tax and branching logic

For prompt-driven adapters, the prompts should be equivalent to:

```text
What is the order amount?
What is the state?
```

### 3. Refactor

Clean up any remaining duplication while keeping the full suite green.
