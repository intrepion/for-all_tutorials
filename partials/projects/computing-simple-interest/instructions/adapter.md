---
partial_kind: adapter-instructions
project: computing-simple-interest
---

# Adapter Instructions

Project-specific adapter instruction fragment for the `computing-simple-interest` adapter repo.

### 1. Red: Add An Adapter-Level Test

In a separate adapter repo, add a failing test that proves the chosen adapter delegates to the core logic correctly.

### 2. Green: Add The Thin Surface Adapter

Add the thinnest possible adapter for the surface you are building:

- collect one principal input string from that surface
- collect one annual-rate input string from that surface
- collect one years input string from that surface
- parse the principal into a whole number that matches the core contract
- parse the annual rate into the exact scaled integer value expected by the core contract
- parse the years value into a whole number that matches the core contract
- pass the parsed values to `calculate_simple_interest`
- pass the parsed years value, parsed annual rate, and returned accrued amount to `format_simple_interest_report`
- return, render, or print the report sentence in the form that surface requires
- keep transport, parsing, and input/output code out of the core interest and rounding logic

For prompt-driven adapters, the prompts should be equivalent to:

```text
Enter the principal:
Enter the rate of interest:
Enter the number of years:
```

### 3. Refactor

Clean up any remaining duplication while keeping the full suite green.

