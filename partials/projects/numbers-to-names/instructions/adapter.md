---
partial_kind: adapter-instructions
project: numbers-to-names
---

# Adapter Instructions

Project-specific adapter instruction fragment for the `numbers-to-names` adapter repo.

### 1. Red: Add An Adapter-Level Test

In a separate adapter repo, add a failing test that proves the chosen adapter delegates to the core logic correctly.

### 2. Green: Add The Thin Surface Adapter

Add the thinnest possible adapter for the surface you are building:

- collect one month-number input string from that surface
- parse the month number into a whole number that matches the core contract
- pass the parsed month number to `lookup_month_name`
- pass the returned month name or `null` to `format_month_lookup_message`
- return, render, or print the returned message in the form that surface requires
- keep transport, parsing, and input/output code out of the core lookup and formatting logic

For prompt-driven adapters, the prompts should be equivalent to:

```text
Please enter the number of the month:
```

### 3. Refactor

Clean up any remaining duplication while keeping the full suite green.

