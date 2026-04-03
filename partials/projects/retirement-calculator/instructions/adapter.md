---
partial_kind: adapter-instructions
project: retirement-calculator
---

# Adapter Instructions

Project-specific adapter instruction fragment for the `retirement-calculator` adapter repo.

### 1. Red: Add An Adapter-Level Test

In a separate adapter repo, add a failing test that proves the chosen adapter delegates to the core logic correctly.

### 2. Green: Add The Thin Surface Adapter

Add the thinnest possible adapter for the surface you are building:

- collect one current-age input string from that surface
- collect one desired-retirement-age input string from that surface
- parse those strings into numeric values that match the core contract
- retrieve the current calendar year from the runtime or host environment
- pass the parsed ages and retrieved current year to `calculate_retirement`
- pass the returned values plus the current year to `format_retirement_report`
- return, render, or print the two lines in the form that surface requires
- keep transport, parsing, clock access, and input/output code out of the core retirement logic

For prompt-driven adapters, the prompts should be equivalent to:

```text
What is your current age?
At what age would you like to retire?
```

### 3. Refactor

Clean up any remaining duplication while keeping the full suite green.

