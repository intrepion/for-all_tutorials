<!-- breadcrumbs:start -->
[for-all_tutorials](../../../README.md) / [Projects](../../README.md) / [Computing Simple Interest](../README.md) / Spec
<!-- breadcrumbs:end -->

# Spec

Canonical project contract for the `computing-simple-interest` project.

## Goal

Build a small project app that introduces:

- converting a prompted principal amount into a numeric value
- converting a prompted annual interest rate into an exact scaled value
- computing accrued value with the simple-interest formula
- making the order of operations explicit in the core logic
- rounding the final accrued amount to the nearest cent with half-up rounding
- separating interest calculation from output formatting
- test-first calculation and formatting logic in small core functions
- thin surface adapters

## Core Logic Contract

The shared core contracts are:

```text
calculate_simple_interest(principal_dollars: integer, annual_rate_tenths_percent: integer, years: integer) -> accrued_amount_cents
format_simple_interest_report(years: integer, annual_rate_tenths_percent: integer, accrued_amount_cents: integer) -> string
```

Canonical behavior:

- treat `principal_dollars` as a positive whole number of dollars
- treat `annual_rate_tenths_percent` as the entered annual rate scaled by `10`, where:
  - `43` means an entered annual rate of `4.3%`
  - `5` means an entered annual rate of `0.5%`
- treat `years` as a positive whole number of years
- `calculate_simple_interest` returns `accrued_amount_cents` using exact integer math plus half-up cent rounding:
  - `principal_cents = principal_dollars * 100`
  - `interest_cents = principal_cents * annual_rate_tenths_percent * years / 1000`, rounded to the nearest whole cent with half-up rounding
  - `accrued_amount_cents = principal_cents + interest_cents`
- `format_simple_interest_report` returns:
  - `After <years> years at <annual_rate_with_exactly_one_decimal_place>%, the investment will be worth $<accrued_amount_with_exactly_two_decimal_places>.`
- the report must preserve the whole-number year count exactly as supplied
- the report must render the rate with exactly one decimal place
- the report must render the accrued amount with exactly two decimal places
- the core logic must use exact scaled integer values rather than floating-point money calculations

Examples:

- `calculate_simple_interest(1500, 43, 4)` returns `175800`
- `format_simple_interest_report(4, 43, 175800)` returns `After 4 years at 4.3%, the investment will be worth $1758.00.`
- `calculate_simple_interest(1, 5, 1)` returns `101`
- `format_simple_interest_report(1, 5, 101)` returns `After 1 years at 0.5%, the investment will be worth $1.01.`

## Non-Goals

This project does not include:

- persistence
- localization
- authentication
- network access
- configuration files
- validation for zero or negative principal amounts, rates, or years
- decimal principal input amounts
- compound interest
- monthly or daily compounding periods
- fetching live rates from external services
- grammatical singular-versus-plural wording changes such as `1 year`
- floating-point money calculations in the core logic

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should adapt the same simple-interest and report-formatting behavior instead of redefining it.

Adapters should parse the entered interest rate into the exact scaled integer value expected by the core logic before calling the core.

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns `calculate_simple_interest` and `format_simple_interest_report`, and adapter repos must not reimplement them.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- tests for `calculate_simple_interest` on the canonical example
- tests for a half-up rounding case where `$1` at `0.5%` for `1` year produces `$1.01`
- tests that `format_simple_interest_report` preserves the exact wording of the canonical sentence
- tests that the annual rate is rendered with exactly one decimal place
- tests that the accrued amount is rendered with exactly two decimal places
- tests for every adapter built in the chosen run that prove it delegates to the core logic correctly
