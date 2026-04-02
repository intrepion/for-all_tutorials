<!-- breadcrumbs:start -->
[for-all_tutorials](../../../README.md) / [Projects](../../README.md) / [Determining Compound Interest](../README.md) / Spec
<!-- breadcrumbs:end -->

# Spec

Canonical project contract for the `determining-compound-interest` project.

## Goal

Build a small project app that introduces:

- converting a prompted principal amount into a numeric value
- converting a prompted annual interest rate into an exact scaled value
- computing accrued value with the compound-interest formula
- incorporating exponent-based growth into the core logic
- rounding the final accrued amount to the nearest cent with half-up rounding
- separating compound-interest calculation from output formatting
- test-first calculation and formatting logic in small core functions
- thin surface adapters

## Core Logic Contract

The shared core contracts are:

```text
calculate_compound_interest(principal_dollars: integer, annual_rate_tenths_percent: integer, years: integer, compounds_per_year: integer) -> accrued_amount_cents
format_compound_interest_report(principal_dollars: integer, annual_rate_tenths_percent: integer, years: integer, compounds_per_year: integer, accrued_amount_cents: integer) -> string
```

Canonical behavior:

- treat `principal_dollars` as a positive whole number of dollars
- treat `annual_rate_tenths_percent` as the entered annual rate scaled by `10`, where:
  - `43` means an entered annual rate of `4.3%`
  - `5` means an entered annual rate of `0.5%`
- treat `years` as a positive whole number of years
- treat `compounds_per_year` as a positive whole number
- `calculate_compound_interest` returns `accrued_amount_cents` by applying the exercise formula:
  - `A = P * ((1 + (r / n))^(n * t))`
  - `P` is `principal_dollars`
  - `r` is `annual_rate_tenths_percent / 1000`
  - `n` is `compounds_per_year`
  - `t` is `years`
- after computing `A` in dollars, round the final accrued amount to the nearest cent with half-up rounding
- the internal implementation may use high-precision decimal or equivalent deterministic math, but the observable output must match the canonical cent-rounded result
- `format_compound_interest_report` returns:
  - `$<principal_dollars> invested at <annual_rate_with_exactly_one_decimal_place>% for <years> years compounded <compounds_per_year> times per year is $<accrued_amount_with_exactly_two_decimal_places>.`
- the report must preserve the whole-number principal, year count, and compounds-per-year count exactly as supplied
- the report must render the rate with exactly one decimal place
- the report must render the accrued amount with exactly two decimal places

Examples:

- `calculate_compound_interest(1500, 43, 6, 4)` returns `193884`
- `format_compound_interest_report(1500, 43, 6, 4, 193884)` returns `$1500 invested at 4.3% for 6 years compounded 4 times per year is $1938.84.`
- `calculate_compound_interest(1, 5, 1, 1)` returns `101`
- `format_compound_interest_report(1, 5, 1, 1, 101)` returns `$1 invested at 0.5% for 1 years compounded 1 times per year is $1.01.`

## Non-Goals

This project does not include:

- persistence
- localization
- authentication
- network access
- configuration files
- validation for zero or negative principal amounts, rates, years, or compound periods
- decimal principal input amounts
- simple interest
- changing contribution amounts over time
- fetching live rates from external services
- grammatical singular-versus-plural wording changes such as `1 year` or `1 time`
- prescribing one exact numeric library or numeric type for every language

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should adapt the same compound-interest and report-formatting behavior instead of redefining it.

Adapters should parse the entered interest rate into the exact scaled integer value expected by the core logic before calling the core.

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns `calculate_compound_interest` and `format_compound_interest_report`, and adapter repos must not reimplement them.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- tests for `calculate_compound_interest` on the canonical example
- tests for a half-up rounding case where `$1` at `0.5%` for `1` year compounded `1` time per year produces `$1.01`
- tests that `format_compound_interest_report` preserves the exact wording of the canonical sentence
- tests that the annual rate is rendered with exactly one decimal place
- tests that the accrued amount is rendered with exactly two decimal places
- tests for every adapter built in the chosen run that prove it delegates to the core logic correctly
