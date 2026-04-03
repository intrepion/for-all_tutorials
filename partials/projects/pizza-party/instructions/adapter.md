---
partial_kind: adapter-instructions
project: pizza-party
---

# Adapter Instructions

Project-specific adapter instruction fragment for the `pizza-party` adapter repo.

### 1. Red: Add An Adapter-Level Test

In a separate adapter repo, add a failing test that proves the chosen adapter delegates to the core logic correctly.

### 2. Green: Add The Thin Surface Adapter

Add the thinnest possible adapter for the surface you are building:

- collect one people-count input string from that surface
- collect one pizza-count input string from that surface
- collect one slices-per-pizza input string from that surface
- parse those strings into numeric values that match the core contract
- pass the parsed values to `calculate_pizza_distribution`
- pass the original numeric values plus the returned distribution to `format_pizza_distribution_report`
- return, render, or print the three lines in the form that surface requires
- keep transport, parsing, and input/output code out of the core pizza-distribution logic

For prompt-driven adapters, the prompts should be equivalent to:

```text
How many people?
How many pizzas do you have?
How many slices per pizza?
```

### 3. Refactor

Clean up any remaining duplication while keeping the full suite green.

