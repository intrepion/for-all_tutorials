---
partial_kind: adapter-instructions
project: months-to-pay-off-a-credit-card
---

# Adapter Instructions

Project-specific adapter instruction fragment for the `months-to-pay-off-a-credit-card` adapter repo.

### 1. Red: Add An Adapter-Level Test

In a separate adapter repo, add a failing test that proves the chosen adapter delegates to the core logic correctly.

### 2. Green: Add The Thin Surface Adapter

Add the thinnest possible adapter for the surface you are building:

- collect one balance value from that surface
- collect one APR value from that surface
- collect one monthly-payment value from that surface
- parse the APR into the exact scaled integer value expected by `calculate_credit_card_payoff_months`
- pass the parsed numeric values to `calculate_credit_card_payoff_months`
- pass the resulting month count to `format_credit_card_payoff_report`
- return, render, or print the formatted result in the form that surface requires
- keep transport, parsing, and input/output code out of the core payoff logic

For prompt-driven adapters, the prompts should be equivalent to:

```text
What is your balance?
What is the APR on the card (as a percent)?
What is the monthly payment you can make?
```

### 3. Refactor

Clean up any remaining duplication while keeping the full suite green.

