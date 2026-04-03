---
partial_kind: project-spec
project: months-to-pay-off-a-credit-card
---

# Spec

Canonical project contract for the `months-to-pay-off-a-credit-card` project.

## Goal

Build a small project app that introduces:

- converting a prompted credit-card balance into a numeric value
- converting a prompted APR into an exact scaled value
- applying the credit-card payoff formula with logarithms and exponentiation
- rounding the calculated payoff duration up to the next whole month
- separating payoff calculation from output formatting
- test-first calculation and formatting logic in small core functions
- thin surface adapters

## Core Logic Contract

The shared core contracts are:

```text
calculate_credit_card_payoff_months(balance_dollars: integer, apr_tenths_percent: integer, monthly_payment_dollars: integer) -> months
format_credit_card_payoff_report(months: integer) -> string
```

Canonical behavior:

- treat `balance_dollars` as a positive whole number of dollars
- treat `apr_tenths_percent` as the entered APR scaled by `10`, where:
  - `120` means an entered APR of `12.0%`
  - `185` means an entered APR of `18.5%`
- treat `monthly_payment_dollars` as a positive whole number of dollars
- `calculate_credit_card_payoff_months` returns `months` by applying the exercise formula:
  - `n = (-1 / 30) * (log(1 + ((b / p) * (1 - ((1 + i)^30)))) / log(1 + i))`
  - `b` is `balance_dollars`
  - `p` is `monthly_payment_dollars`
  - `i` is the daily rate, computed as `(apr_tenths_percent / 1000) / 365`
- after computing `n`, round the result up to the next whole month using ceiling rounding
- the internal implementation may use high-precision decimal or equivalent deterministic math, but the observable output must match the canonical whole-month result
- `format_credit_card_payoff_report` returns:
  - `It will take you <months> months to pay off this card.`
- the report must preserve the whole-number month count exactly as supplied

Examples:

- `calculate_credit_card_payoff_months(5000, 120, 100)` returns `70`
- `format_credit_card_payoff_report(70)` returns `It will take you 70 months to pay off this card.`
- `calculate_credit_card_payoff_months(1000, 120, 100)` returns `11`
- `calculate_credit_card_payoff_months(1200, 180, 100)` returns `14`
- `format_credit_card_payoff_report(11)` returns `It will take you 11 months to pay off this card.`

## Non-Goals

This project does not include:

- persistence
- localization
- authentication
- network access
- configuration files
- validation for zero or negative balances, APR values, or monthly payments
- decimal balance or monthly-payment input amounts
- zero-APR payoff behavior outside the exercise formula
- impossible-payoff scenarios where the monthly payment is too low for the formula domain
- full amortization schedules
- extra or changing payments over time

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should adapt the same payoff calculation and report-formatting behavior instead of redefining it.

Adapters should parse the entered APR into the exact scaled integer value expected by the core logic before calling the core.

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns `calculate_credit_card_payoff_months` and `format_credit_card_payoff_report`, and adapter repos must not reimplement them.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- tests for `calculate_credit_card_payoff_months` on the canonical example
- tests for a second APR-sensitive case such as `(1200, 180, 100) -> 14`
- tests that prove the result is rounded up to the next whole month, such as `(1000, 120, 100) -> 11`
- tests that `format_credit_card_payoff_report` preserves the exact canonical sentence
- tests for every adapter built in the chosen run that prove it delegates to the core logic correctly

