<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../README.md) / [Projects](../../../README.md) / [Determining Compound Interest](../../README.md) / [Tutorial](../README.md) / adapter.md
<!-- breadcrumbs:end -->

# Adapter Walkthrough

Project-specific red, green, refactor sequence for the `determining-compound-interest` adapter repo.

### 1. Red: Add An Adapter-Level Test

In a separate adapter repo, add a failing test that proves the chosen adapter delegates to the core logic correctly.

### 2. Green: Add The Thin Surface Adapter

Add the thinnest possible adapter for the surface you are building:

- collect one principal input string from that surface
- collect one annual-rate input string from that surface
- collect one years input string from that surface
- collect one compounds-per-year input string from that surface
- parse the principal into a whole number that matches the core contract
- parse the annual rate into the exact scaled integer value expected by the core contract
- parse the years value into a whole number that matches the core contract
- parse the compounds-per-year value into a whole number that matches the core contract
- pass the parsed values to `calculate_compound_interest`
- pass the parsed principal, parsed annual rate, parsed years, parsed compounds-per-year value, and returned accrued amount to `format_compound_interest_report`
- return, render, or print the report sentence in the form that surface requires
- keep transport, parsing, and input/output code out of the core compound-interest and rounding logic

For prompt-driven adapters, the prompts should be equivalent to:

```text
What is the principal amount?
What is the rate?
What is the number of years?
What is the number of times the interest is compounded per year?
```

### 3. Refactor

Clean up any remaining duplication while keeping the full suite green.
