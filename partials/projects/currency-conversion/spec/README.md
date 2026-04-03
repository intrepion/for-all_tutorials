---
partial_kind: project-spec
project: currency-conversion
---

# Spec

Canonical project contract for the `currency-conversion` project.

## Goal

Build a small project app that introduces:

- converting a prompted euro amount into a numeric value
- converting a prompted exchange-rate string into an exact scaled value
- converting euros to U.S. dollars with the provided exchange rate
- rounding the final U.S. dollar amount to the nearest cent with half-up rounding
- separating conversion calculation from output formatting
- test-first calculation and formatting logic in small core functions
- thin surface adapters

## Core Logic Contract

The shared core contracts are:

```text
calculate_currency_conversion(amount_euros: integer, exchange_rate_hundredths: integer) -> usd_cents
format_currency_conversion_report(amount_euros: integer, exchange_rate_hundredths: integer, usd_cents: integer) -> string[]
```

Canonical behavior:

- treat `amount_euros` as a positive whole number of euros
- treat `exchange_rate_hundredths` as the entered exchange rate scaled by `100`, where:
  - `13751` means an entered exchange rate of `137.51`
  - `10050` means an entered exchange rate of `100.50`
- use the exercise formula with a fixed `rate_to` value of `100`
- `calculate_currency_conversion` returns `usd_cents` using exact integer math plus half-up cent rounding:
  - raw cents = `(amount_euros * exchange_rate_hundredths) / 100`
  - `usd_cents` is the raw result rounded to the nearest whole cent with half-up rounding
- `format_currency_conversion_report` returns two lines in this exact order:
  - `<amount_euros> euros at an exchange rate of <exchange_rate_with_exactly_two_decimal_places> is`
  - `<usd_amount_with_exactly_two_decimal_places> U.S. dollars.`
- the first line must preserve the whole-number euro amount exactly as supplied
- the first line must render the exchange rate with exactly two decimal places
- the second line must render the converted U.S. dollar amount with exactly two decimal places
- the core logic must use exact scaled integer values rather than floating-point money calculations

Examples:

- `calculate_currency_conversion(81, 13751)` returns `11138`
- `format_currency_conversion_report(81, 13751, 11138)` returns:
  - `81 euros at an exchange rate of 137.51 is`
  - `111.38 U.S. dollars.`
- `calculate_currency_conversion(1, 10050)` returns `101`
- `format_currency_conversion_report(1, 10050, 101)` returns:
  - `1 euros at an exchange rate of 100.50 is`
  - `1.01 U.S. dollars.`

## Non-Goals

This project does not include:

- persistence
- localization
- authentication
- network access
- configuration files
- validation for zero or negative euro amounts or exchange rates
- decimal euro input amounts
- fetching live exchange rates from external services
- converting currencies other than euros to U.S. dollars
- grammatical singular-versus-plural wording changes such as `1 euro`
- floating-point money calculations in the core logic

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should adapt the same currency-conversion and report-formatting behavior instead of redefining it.

Adapters should parse the entered exchange rate into the exact scaled integer value expected by the core logic before calling the core.

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns `calculate_currency_conversion` and `format_currency_conversion_report`, and adapter repos must not reimplement them.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- tests for `calculate_currency_conversion` on the canonical example
- tests for a half-up rounding case where `1` euro at an exchange rate of `100.50` produces `1.01` U.S. dollars
- tests that `format_currency_conversion_report` preserves the exact two-line order and wording
- tests that the exchange rate is rendered with exactly two decimal places
- tests that the converted U.S. dollar amount is rendered with exactly two decimal places
- tests for every adapter built in the chosen run that prove it delegates to the core logic correctly

