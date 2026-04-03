---
partial_kind: adapter-instructions
project: blood-alcohol-calculator
---

# Adapter Instructions

Project-specific adapter instruction fragment for the `blood-alcohol-calculator` adapter repo.

### 1. Red: Add An Adapter-Level Test

In a separate adapter repo, add a failing test that proves the chosen adapter delegates to the core logic correctly.

### 2. Green: Add The Thin Surface Adapter

Add the thinnest possible adapter for the surface you are building:

- collect one weight input string from that surface
- collect one gender input string from that surface
- collect one drink-count input string from that surface
- collect one pure-alcohol-per-drink input string from that surface
- collect one hours-since-last-drink input string from that surface
- parse each input into the exact numeric or categorical value expected by the core contract
- pass the parsed values to `calculate_bac_hundredths`
- pass the returned BAC value to `is_legal_to_drive_with_bac`
- pass the returned BAC value and legality result to `format_bac_report`
- return, render, or print the returned lines in the form that surface requires
- keep transport, parsing, and input/output code out of the core BAC and legality logic

For prompt-driven adapters, the prompts should be equivalent to:

```text
What is your weight?
What is your gender?
How many drinks have you had?
How many ounces of pure alcohol were in each drink?
How many hours has it been since your last drink?
```

### 3. Refactor

Clean up any remaining duplication while keeping the full suite green.

