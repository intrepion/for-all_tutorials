---
partial_kind: project-spec
project: blood-alcohol-calculator
---

# Spec

Canonical project contract for the `blood-alcohol-calculator` project.

## Goal

Build a small project app that introduces:

- converting prompted weight, drinks, alcohol, and time inputs into numeric values
- choosing a distribution ratio from the supplied gender
- applying the BAC formula in a deterministic way
- rounding the calculated BAC to exactly two decimal places
- comparing the rounded BAC to the fixed legal threshold of `0.08`
- separating BAC calculation, legal determination, and output formatting
- test-first calculation and formatting logic in small core functions
- thin surface adapters

## Core Logic Contract

The shared core contracts are:

```text
calculate_bac_hundredths(weight_pounds: integer, gender: string, number_of_drinks: integer, pure_alcohol_ounces_per_drink_hundredths: integer, hours_since_last_drink: integer) -> bac_hundredths
is_legal_to_drive_with_bac(bac_hundredths: integer) -> boolean
format_bac_report(bac_hundredths: integer, is_legal_to_drive: boolean) -> string[]
```

Canonical behavior:

- treat `weight_pounds` as a positive whole number of pounds
- treat `gender` as one of two exact values:
  - `male`, which uses a distribution ratio of `0.73`
  - `female`, which uses a distribution ratio of `0.66`
- treat `number_of_drinks` as a positive whole number
- treat `pure_alcohol_ounces_per_drink_hundredths` as the number of pure alcohol ounces in each drink, scaled by `100`, where:
  - `60` means `0.60` ounces of pure alcohol per drink
  - `75` means `0.75` ounces of pure alcohol per drink
- treat `hours_since_last_drink` as a non-negative whole number of hours
- for this curriculum, the alcohol input is interpreted as pure alcohol ounces per drink, not ABV percentage, because the BAC formula requires total alcohol consumed in ounces
- `calculate_bac_hundredths`:
  - computes total alcohol consumed in ounces as:
    - `A = number_of_drinks * pure_alcohol_ounces_per_drink_hundredths / 100`
  - applies the BAC formula:
    - `BAC = ((A * 5.14) / (weight_pounds * r)) - (0.015 * hours_since_last_drink)`
  - rounds the resulting BAC to exactly two decimal places using half-up rounding
  - returns the rounded BAC as `bac_hundredths`, where:
    - `8` means `0.08`
    - `6` means `0.06`
  - if the computed BAC is negative, returns `0`
- `is_legal_to_drive_with_bac` returns:
  - `true` when `bac_hundredths` is less than `8`
  - `false` when `bac_hundredths` is `8` or greater
- `format_bac_report` returns two lines in this exact order:
  - `Your BAC is <bac_with_exactly_two_decimal_places>`
  - `It is legal for you to drive.` when `is_legal_to_drive` is `true`
  - `It is not legal for you to drive.` when `is_legal_to_drive` is `false`

Examples:

- `calculate_bac_hundredths(175, "male", 4, 60, 1)` returns `8`
- `is_legal_to_drive_with_bac(8)` returns `false`
- `format_bac_report(8, false)` returns:
  - `Your BAC is 0.08`
  - `It is not legal for you to drive.`
- `calculate_bac_hundredths(175, "female", 4, 60, 1)` returns `9`
- `calculate_bac_hundredths(180, "male", 4, 60, 2)` returns `6`
- `format_bac_report(6, true)` returns:
  - `Your BAC is 0.06`
  - `It is legal for you to drive.`
- `calculate_bac_hundredths(200, "male", 1, 60, 10)` returns `0`

## Non-Goals

This project does not include:

- persistence
- localization
- authentication
- network access
- configuration files
- validation for zero or negative weight or drink counts
- gender normalization, trimming, or case folding
- inferring pure alcohol ounces from beverage size and ABV percentage
- fractional hours since the last drink
- regional BAC thresholds other than the fixed legal threshold of `0.08`
- medical advice or safety guidance beyond the exercise's legal/not-legal output

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should adapt the same BAC-calculation, legal-determination, and report-formatting behavior instead of redefining it.

Adapters should parse each prompted input into the exact numeric or categorical values expected by the core logic before calling the core.

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns `calculate_bac_hundredths`, `is_legal_to_drive_with_bac`, and `format_bac_report`, and adapter repos must not reimplement them.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- tests for `calculate_bac_hundredths` on the canonical not-legal example
- tests that the female distribution ratio produces a higher BAC than the equivalent male case
- tests that negative computed BAC values clamp to `0`
- tests for the exact legal threshold case `0.08`, which must be not legal
- tests for a below-threshold case such as `0.06`, which must be legal
- tests that `format_bac_report` preserves the exact not-legal two-line report
- tests that `format_bac_report` preserves the exact legal two-line report
- tests for every adapter built in the chosen run that prove it delegates to the core logic correctly

