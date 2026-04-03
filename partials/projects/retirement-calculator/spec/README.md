---
partial_kind: project-spec
project: retirement-calculator
---

# Spec

Canonical project contract for the `retirement-calculator` project.

## Goal

Build a small project app that introduces:

- converting two prompted age inputs into numeric values
- combining those values with the current year supplied by the adapter
- calculating years-left and retirement-year values
- separating retirement calculation from output formatting
- test-first calculation and formatting logic in small core functions
- thin surface adapters

## Core Logic Contract

The shared core contracts are:

```text
calculate_retirement(current_age: integer, desired_retirement_age: integer, current_year: integer) -> { years_left, retirement_year }
format_retirement_report(current_year: integer, years_left: integer, retirement_year: integer) -> string[]
```

Canonical behavior:

- treat `current_age` and `desired_retirement_age` as whole-number ages
- treat `current_year` as the calendar year supplied by the adapter
- `calculate_retirement` returns:
  - `years_left = desired_retirement_age - current_age`
  - `retirement_year = current_year + years_left`
- for this project, `desired_retirement_age` is assumed to be greater than `current_age`
- `format_retirement_report` returns two lines in this exact order:
  - `You have <years_left> year left until you can retire.` when `years_left` is `1`
  - `You have <years_left> years left until you can retire.` when `years_left` is greater than `1`
  - `It's <current_year>, so you can retire in <retirement_year>.`
- preserve the supplied `current_year` and computed `retirement_year` exactly in the rendered output

Examples:

- `calculate_retirement(25, 65, 2015)` returns `{ years_left: 40, retirement_year: 2055 }`
- `format_retirement_report(2015, 40, 2055)` returns:
  - `You have 40 years left until you can retire.`
  - `It's 2015, so you can retire in 2055.`
- `calculate_retirement(64, 65, 2015)` returns `{ years_left: 1, retirement_year: 2016 }`
- `format_retirement_report(2015, 1, 2016)` returns:
  - `You have 1 year left until you can retire.`
  - `It's 2015, so you can retire in 2016.`

## Non-Goals

This project does not include:

- persistence
- localization
- authentication
- network access
- configuration files
- validation for negative ages
- handling cases where the desired retirement age is less than or equal to the current age
- month- or day-level retirement timing
- timezone-specific date calculations beyond retrieving the current calendar year

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should adapt the same retirement-calculation and report-formatting behavior instead of redefining it.

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns `calculate_retirement` and `format_retirement_report`, and adapter repos must not reimplement them.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- tests for `calculate_retirement` on the canonical example
- tests for the singular `1 year` case
- tests that `format_retirement_report` preserves the exact line order and wording
- tests that the supplied `current_year` and computed `retirement_year` appear unchanged in the second line
- tests for every adapter built in the chosen run that prove it delegates to the core logic correctly

