---
partial_kind: adapter-instructions
project: currency-conversion
---

# Adapter Instructions

Project-specific adapter instruction fragment for the `currency-conversion` adapter repo.

### 1. Red: Add An Adapter-Level Test

In a separate adapter repo, add a failing test that proves the chosen adapter delegates to the core logic correctly.

### 2. Green: Add The Thin Surface Adapter

Add the thinnest possible adapter for the surface you are building:

- collect one euro-amount input string from that surface
- collect one exchange-rate input string from that surface
- parse the euro amount into a whole number that matches the core contract
- parse the exchange rate into the exact scaled integer value expected by the core contract
- pass the parsed values to `calculate_currency_conversion`
- pass the parsed euro amount, parsed exchange rate, and returned U.S. dollar amount to `format_currency_conversion_report`
- return, render, or print the two lines in the form that surface requires
- keep transport, parsing, and input/output code out of the core conversion and rounding logic

For prompt-driven adapters, the prompts should be equivalent to:

```text
How many euros are you exchanging?
What is the exchange rate?
```

### 3. Refactor

Clean up any remaining duplication while keeping the full suite green.

