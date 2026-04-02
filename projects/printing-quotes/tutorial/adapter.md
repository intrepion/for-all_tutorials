<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../README.md) / [Projects](../../../README.md) / [Printing Quotes](../../README.md) / [Tutorial](../README.md) / adapter.md
<!-- breadcrumbs:end -->

# Adapter Walkthrough

Project-specific red, green, refactor sequence for the `printing-quotes` adapter repo.

### 1. Red: Add An Adapter-Level Test

In a separate adapter repo, add a failing test that proves the chosen adapter delegates to the core logic correctly.

### 2. Green: Add The Thin Surface Adapter

Add the thinnest possible adapter for the surface you are building:

- collect one quote string from that surface
- collect one author string from that surface
- pass both values to `format_attributed_quote`
- return, render, or print the result in the form that surface requires
- keep transport, parsing, and input/output code out of the core quote-formatting logic

For prompt-driven adapters, the prompts should be equivalent to:

```text
What is the quote?
Who said it?
```

The adapter must pass the collected author and quote through exactly as entered.

### 3. Refactor

Clean up any remaining duplication while keeping the full suite green.
