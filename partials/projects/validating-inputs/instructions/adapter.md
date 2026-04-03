---
partial_kind: adapter-instructions
project: validating-inputs
---

# Adapter Instructions

Project-specific adapter instruction fragment for the `validating-inputs` adapter repo.

### 1. Red: Add An Adapter-Level Test

In a separate adapter repo, add a failing test that proves the chosen adapter delegates to the core logic correctly.

### 2. Green: Add The Thin Surface Adapter

Add the thinnest possible adapter for the surface you are building:

- collect one first-name string from that surface
- collect one last-name string from that surface
- collect one ZIP-code string from that surface
- collect one employee-ID string from that surface
- pass those four raw strings to `collect_validation_errors`
- pass the resulting errors to `format_validation_report`
- return, render, or print the formatted result in the form that surface requires
- keep transport, parsing, and input/output code out of the core validation logic

For prompt-driven adapters, the prompts should be equivalent to:

```text
Enter the first name:
Enter the last name:
Enter the ZIP code:
Enter an employee ID:
```

### 3. Refactor

Clean up any remaining duplication while keeping the full suite green.

