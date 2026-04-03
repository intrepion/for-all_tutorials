<!-- breadcrumbs:start -->
[for-all_tutorials](../../../README.md) / [Projects](../../README.md) / [Multistate Sales Tax Calculator](../README.md) / Spec
<!-- breadcrumbs:end -->

# Spec

Canonical project contract for the `multistate-sales-tax-calculator` project.

## Goal

Build a small project app that introduces:

- converting a prompted order amount into an exact money value
- branching first on state and then on county for Wisconsin orders
- applying a fixed `5%` Wisconsin base sales tax
- applying additional Wisconsin county tax for `Eau Claire` and `Dunn`
- applying a fixed `8%` Illinois sales tax
- omitting tax for all other states
- rounding tax to the nearest cent with half-up rounding
- separating tax calculation from output formatting
- test-first calculation and formatting logic in small core functions
- thin surface adapters

## Core Logic Contract

The shared core contracts are:

```text
calculate_multistate_sales_tax(order_amount_cents: integer, state: string, county: string | null) -> { tax_cents, total_cents, tax_applied, effective_rate_thousandths }
format_multistate_sales_tax_report(summary) -> string[]
```

Canonical behavior:

- treat `order_amount_cents` as a positive whole number of cents
- treat `state` as the exact state value supplied to the core logic
- treat `county` as the exact county value supplied to the core logic, or `null` when no county applies
- `calculate_multistate_sales_tax` returns:
  - when `state` is exactly `Wisconsin`:
    - base Wisconsin rate is `5%`
    - if `county` is exactly `Eau Claire`, add `0.5%`
    - if `county` is exactly `Dunn`, add `0.4%`
    - other Wisconsin counties add no extra county tax
    - `effective_rate_thousandths` is:
      - `50` for Wisconsin with no county surcharge
      - `55` for `Eau Claire`
      - `54` for `Dunn`
    - `tax_applied = true`
  - when `state` is exactly `Illinois`:
    - `effective_rate_thousandths = 80`
    - `tax_applied = true`
  - otherwise:
    - `effective_rate_thousandths = 0`
    - `tax_applied = false`
  - `tax_cents = order_amount_cents * effective_rate_thousandths / 1000`, rounded to the nearest cent with half-up rounding
  - `total_cents = order_amount_cents + tax_cents`
- `format_multistate_sales_tax_report` returns:
  - two lines in this exact order when `tax_applied` is `true`:
    - `The tax is $<tax_with_exactly_two_decimal_places>.`
    - `The total is $<total_with_exactly_two_decimal_places>.`
  - one line when `tax_applied` is `false`:
    - `The total is $<total_with_exactly_two_decimal_places>.`
- currency values must always render with a leading `$` and exactly two decimal places
- the core logic must use exact cent-based values rather than floating-point money values

Examples:

- `calculate_multistate_sales_tax(1000, "Wisconsin", null)` returns `{ tax_cents: 50, total_cents: 1050, tax_applied: true, effective_rate_thousandths: 50 }`
- `format_multistate_sales_tax_report({ tax_cents: 50, total_cents: 1050, tax_applied: true, effective_rate_thousandths: 50 })` returns:
  - `The tax is $0.50.`
  - `The total is $10.50.`
- `calculate_multistate_sales_tax(1000, "Wisconsin", "Eau Claire")` returns `{ tax_cents: 55, total_cents: 1055, tax_applied: true, effective_rate_thousandths: 55 }`
- `calculate_multistate_sales_tax(1000, "Wisconsin", "Dunn")` returns `{ tax_cents: 54, total_cents: 1054, tax_applied: true, effective_rate_thousandths: 54 }`
- `calculate_multistate_sales_tax(1000, "Illinois", null)` returns `{ tax_cents: 80, total_cents: 1080, tax_applied: true, effective_rate_thousandths: 80 }`
- `calculate_multistate_sales_tax(1000, "Minnesota", null)` returns `{ tax_cents: 0, total_cents: 1000, tax_applied: false, effective_rate_thousandths: 0 }`

## Non-Goals

This project does not include:

- persistence
- localization
- authentication
- network access
- configuration files
- validation for zero or negative order amounts
- state or county normalization, trimming, or case folding
- county-level tax outside Wisconsin
- city-level or additional jurisdiction tax rules
- discounts, shipping, or itemized checkout lines
- alternate state tax rates beyond the fixed Wisconsin and Illinois rules
- floating-point money calculations in the core logic

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should adapt the same multistate tax-calculation and report-formatting behavior instead of redefining it.

Adapters should parse the prompted order amount into exact cents before calling the core logic. Adapters should only prompt for county when the state is exactly `Wisconsin`.

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns `calculate_multistate_sales_tax` and `format_multistate_sales_tax_report`, and adapter repos must not reimplement them.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- tests for the canonical Wisconsin example with no county surcharge
- tests for the `Eau Claire` county surcharge case
- tests for the `Dunn` county surcharge case
- tests for the Illinois `8%` case
- tests for an untaxed other-state case
- tests that `format_multistate_sales_tax_report` preserves the taxed two-line branch exactly
- tests that `format_multistate_sales_tax_report` preserves the untaxed one-line branch exactly
- tests for every adapter built in the chosen run that prove it delegates to the core logic correctly
