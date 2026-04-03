---
partial_kind: adapter-instructions
project: multistate-sales-tax-calculator
---

# Adapter Instructions

Project-specific adapter instruction fragment for the `multistate-sales-tax-calculator` adapter repo.

### 1. Red: Add An Adapter-Level Test

In a separate adapter repo, add a failing test that proves the chosen adapter delegates to the core logic correctly.

### 2. Green: Add The Thin Surface Adapter

Add the thinnest possible adapter for the surface you are building:

- collect one order-amount input string from that surface
- collect one state input string from that surface
- if the state is exactly `Wisconsin`, collect one county input string from that surface
- parse the order amount into exact cents that match the core contract
- pass the parsed order amount, state, and county or `null` to `calculate_multistate_sales_tax`
- pass the returned summary to `format_multistate_sales_tax_report`
- return, render, or print the returned lines in the form that surface requires
- keep transport, parsing, and input/output code out of the core nested tax logic

For prompt-driven adapters, the prompts should be equivalent to:

```text
What is the order amount?
What state do you live in?
```

and, only when the state is `Wisconsin`:

```text
What county do you live in?
```

### 3. Refactor

Clean up any remaining duplication while keeping the full suite green.

