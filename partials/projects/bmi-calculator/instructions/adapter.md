---
partial_kind: adapter-instructions
project: bmi-calculator
---

# Adapter Instructions

Project-specific adapter instruction fragment for the `bmi-calculator` adapter repo.

### 1. Red: Add An Adapter-Level Test

In a separate adapter repo, add a failing test that proves the chosen adapter delegates to the core logic correctly.

### 2. Green: Add The Thin Surface Adapter

Add the thinnest possible adapter for the surface you are building:

- collect one weight input string from that surface
- collect one height input string from that surface
- parse the weight into a whole number that matches the core contract
- parse the height into a whole number that matches the core contract
- pass the parsed values to `calculate_bmi_tenths`
- pass the returned BMI value to `classify_bmi`
- pass the returned BMI value and classification to `format_bmi_report`
- return, render, or print the returned lines in the form that surface requires
- keep transport, parsing, and input/output code out of the core BMI and classification logic

For prompt-driven adapters, the prompts should be equivalent to:

```text
What is your weight?
What is your height?
```

### 3. Refactor

Clean up any remaining duplication while keeping the full suite green.

