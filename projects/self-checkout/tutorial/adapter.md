<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../README.md) / [Projects](../../../README.md) / [Self-Checkout](../../README.md) / [Tutorial](../README.md) / adapter.md
<!-- breadcrumbs:end -->

# Adapter Walkthrough

Project-specific red, green, refactor sequence for the `self-checkout` adapter repo.

### 1. Red: Add An Adapter-Level Test

In a separate adapter repo, add a failing test that proves the chosen adapter delegates to the core logic correctly.

### 2. Green: Add The Thin Surface Adapter

Add the thinnest possible adapter for the surface you are building:

- collect one price input string and one quantity input string for each of the three items
- parse each price string into an exact cent value that matches the core contract
- parse each quantity string into a whole number that matches the core contract
- pass the three ordered items to `calculate_checkout_summary`
- pass the returned summary to `format_checkout_report`
- return, render, or print the six lines in the form that surface requires
- keep transport, parsing, and input/output code out of the core checkout and tax logic

For prompt-driven adapters, the prompts should be equivalent to:

```text
Enter the price of item 1:
Enter the quantity of item 1:
Enter the price of item 2:
Enter the quantity of item 2:
Enter the price of item 3:
Enter the quantity of item 3:
```

### 3. Refactor

Clean up any remaining duplication while keeping the full suite green.
