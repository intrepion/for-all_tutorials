---
partial_kind: project-spec
project: tax-calculator
---

# Spec

Canonical project contract for the `tax-calculator` project.

## Goal

Build a small project app that introduces:

- converting a prompted order amount into an exact money value
- branching on a single state-specific tax rule
- applying a fixed `5.5%` tax rate only when the state is `WI`
- rounding tax to the nearest cent with half-up rounding
- separating tax calculation from output formatting
- test-first calculation and formatting logic in small core functions
- thin surface adapters

## Core Logic Contract

The shared core contracts are:

```text
calculate_tax_for_order(order_amount_cents: integer, state: string) -> { subtotal_cents, tax_cents, total_cents, tax_applied }
format_tax_report(summary) -> string[]
```

Canonical behavior:

- treat `order_amount_cents` as a positive whole number of cents
- treat `state` as the exact state value supplied to the core logic
- `calculate_tax_for_order` returns:
  - `subtotal_cents = order_amount_cents`
  - if `state` is exactly `WI`:
    - `tax_applied = true`
    - `tax_cents = subtotal_cents * 5.5%`, rounded to the nearest cent with half-up rounding
    - `total_cents = subtotal_cents + tax_cents`
  - otherwise:
    - `tax_applied = false`
    - `tax_cents = 0`
    - `total_cents = subtotal_cents`
- `format_tax_report` returns:
  - three lines in this exact order when `tax_applied` is `true`:
    - `The subtotal is $<subtotal_with_exactly_two_decimal_places>.`
    - `The tax is $<tax_with_exactly_two_decimal_places>.`
    - `The total is $<total_with_exactly_two_decimal_places>.`
  - one line when `tax_applied` is `false`:
    - `The total is $<total_with_exactly_two_decimal_places>.`
- currency values must always render with a leading `$` and exactly two decimal places
- the core logic must use exact cent-based values rather than floating-point money values

Examples:

- `calculate_tax_for_order(1000, "WI")` returns `{ subtotal_cents: 1000, tax_cents: 55, total_cents: 1055, tax_applied: true }`
- `format_tax_report({ subtotal_cents: 1000, tax_cents: 55, total_cents: 1055, tax_applied: true })` returns:
  - `The subtotal is $10.00.`
  - `The tax is $0.55.`
  - `The total is $10.55.`
- `calculate_tax_for_order(1000, "MN")` returns `{ subtotal_cents: 1000, tax_cents: 0, total_cents: 1000, tax_applied: false }`
- `format_tax_report({ subtotal_cents: 1000, tax_cents: 0, total_cents: 1000, tax_applied: false })` returns:
  - `The total is $10.00.`

## Non-Goals

This project does not include:

- persistence
- localization
- authentication
- network access
- configuration files
- validation for zero or negative order amounts
- state-name normalization, trimming, or case folding
- county, city, or multi-jurisdiction tax rules
- discounts, shipping, or itemized checkout lines
- alternate tax rates beyond the fixed `5.5%` Wisconsin rule
- floating-point money calculations in the core logic

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should adapt the same tax-calculation and report-formatting behavior instead of redefining it.

Adapters should parse the prompted order amount into exact cents before calling the core logic.

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns `calculate_tax_for_order` and `format_tax_report`, and adapter repos must not reimplement them.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- tests for `calculate_tax_for_order` on the canonical Wisconsin example
- tests for a non-Wisconsin case where tax is omitted entirely
- tests for a tax-rounding case where a `$1.00` Wisconsin subtotal produces `$0.06` tax
- tests that `format_tax_report` preserves the taxed three-line branch exactly
- tests that `format_tax_report` preserves the untaxed one-line branch exactly
- tests for every adapter built in the chosen run that prove it delegates to the core logic correctly

