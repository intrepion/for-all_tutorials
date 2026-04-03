---
partial_kind: adapter-instructions
project: handling-bad-input
---

# Adapter Instructions

Project-specific adapter instruction fragment for the `handling-bad-input` adapter repo.

### 1. Red: Add An Adapter-Level Test

In a separate adapter repo, add a failing test that proves the chosen adapter retries on invalid input and delegates to the core logic correctly.

### 2. Green: Add The Thin Surface Adapter

Add the thinnest possible adapter for the surface you are building:

- prompt for one rate-of-return string
- pass the raw input to `parse_rate_of_return`
- when the parse result is invalid:
  - render `format_invalid_rate_message`
  - prompt again
- when the parse result is valid:
  - pass the parsed rate to `calculate_years_to_double`
  - pass the resulting year count to `format_rule_of_72_report`
  - return, render, or print the formatted result in the form that surface requires
- keep the retry loop, transport, and input/output code out of the core calculation logic

For prompt-driven adapters, the prompt should be equivalent to:

```text
What is the rate of return?
```

### 3. Refactor

Clean up any remaining duplication while keeping the full suite green.

