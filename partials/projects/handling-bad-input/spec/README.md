---
partial_kind: project-spec
project: handling-bad-input
---

# Spec

Canonical project contract for the `handling-bad-input` project.

## Goal

Build a small project app that introduces:

- validating one prompted rate-of-return input before using it
- retrying on invalid input without exiting the program
- applying the rule-of-72 calculation in a small core function
- rounding the estimated years to the nearest whole number
- separating parsing, calculation, and output formatting
- test-first validation, calculation, and formatting logic in small core functions
- thin surface adapters

## Core Logic Contract

The shared core contracts are:

```text
parse_rate_of_return(rate_input: string) -> rate_of_return | null
calculate_years_to_double(rate_of_return: integer) -> years
format_invalid_rate_message() -> string
format_rule_of_72_report(years: integer) -> string
```

Canonical behavior:

- treat `rate_input` as a plain-text ASCII string
- do not trim, normalize, or rewrite `rate_input` before validation
- `parse_rate_of_return` returns:
  - `null` when `rate_input` is empty
  - `null` when `rate_input` contains any non-digit character
  - `null` when `rate_input` parses to `0`
  - `null` when `rate_input` parses to a negative value
  - the parsed positive whole-number percentage otherwise
- `calculate_years_to_double` returns `years` by applying the rule of 72:
  - `years = 72 / rate_of_return`
  - round the result to the nearest whole year using half-up rounding
- `format_invalid_rate_message` returns:
  - `Sorry. That's not a valid input.`
- `format_rule_of_72_report` returns:
  - `It will take <years> years to double your initial investment.`

Examples:

- `parse_rate_of_return("0")` returns `null`
- `parse_rate_of_return("ABC")` returns `null`
- `parse_rate_of_return("4")` returns `4`
- `calculate_years_to_double(4)` returns `18`
- `calculate_years_to_double(11)` returns `7`
- `format_invalid_rate_message()` returns `Sorry. That's not a valid input.`
- `format_rule_of_72_report(18)` returns `It will take 18 years to double your initial investment.`

## Non-Goals

This project does not include:

- persistence
- localization
- authentication
- network access
- configuration files
- decimal rate-of-return input values
- percentage symbols in the input
- zero-rate or negative-rate calculation behavior
- compound-interest formulas
- investment schedules beyond the single rule-of-72 estimate

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should adapt the same rate parsing, rule-of-72 calculation, and report-formatting behavior instead of redefining it.

Adapters should keep prompting until `parse_rate_of_return` returns a valid positive whole number. After each invalid value, the adapter should render the canonical invalid-input message and prompt again.

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns `parse_rate_of_return`, `calculate_years_to_double`, `format_invalid_rate_message`, and `format_rule_of_72_report`, and adapter repos must not reimplement them.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- tests that `parse_rate_of_return` rejects `0`
- tests that `parse_rate_of_return` rejects non-numeric input such as `ABC`
- tests that `parse_rate_of_return` accepts a positive whole number such as `4`
- tests for `calculate_years_to_double` on the canonical `4 -> 18` example
- tests for a half-up rounding case such as `11 -> 7`
- tests that `format_invalid_rate_message` preserves the exact canonical sentence
- tests that `format_rule_of_72_report` preserves the exact canonical sentence
- tests for every adapter built in the chosen run that prove it retries on invalid input and delegates to the core logic correctly

