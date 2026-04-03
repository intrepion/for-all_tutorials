<!-- breadcrumbs:start -->
[for-all_tutorials](../../../README.md) / [Projects](../../README.md) / [BMI Calculator](../README.md) / Spec
<!-- breadcrumbs:end -->

# Spec

Canonical project contract for the `bmi-calculator` project.

## Goal

Build a small project app that introduces:

- converting prompted weight and height into numeric values
- applying the BMI formula in a deterministic way
- rounding BMI to exactly one decimal place
- comparing the rounded BMI to the fixed ideal range from `18.5` to `25.0`
- branching between ideal, underweight, and overweight messages
- separating BMI calculation, classification, and output formatting
- test-first calculation and formatting logic in small core functions
- thin surface adapters

## Core Logic Contract

The shared core contracts are:

```text
calculate_bmi_tenths(weight_pounds: integer, height_inches: integer) -> bmi_tenths
classify_bmi(bmi_tenths: integer) -> classification
format_bmi_report(bmi_tenths: integer, classification: string) -> string[]
```

Canonical behavior:

- treat `weight_pounds` as a positive whole number of pounds
- treat `height_inches` as a positive whole number of inches
- `calculate_bmi_tenths`:
  - applies the BMI formula:
    - `bmi = (weight / (height * height)) * 703`
  - rounds the result to exactly one decimal place using half-up rounding
  - returns the rounded BMI as `bmi_tenths`, where:
    - `195` means `19.5`
    - `325` means `32.5`
- `classify_bmi` returns:
  - `ideal` when `bmi_tenths` is from `185` through `250`, inclusive
  - `underweight` when `bmi_tenths` is less than `185`
  - `overweight` when `bmi_tenths` is greater than `250`
- `format_bmi_report` returns two lines in this exact order:
  - `Your BMI is <bmi_with_exactly_one_decimal_place>.`
  - `You are within the ideal weight range.` when `classification` is `ideal`
  - `You are underweight. You should see your doctor.` when `classification` is `underweight`
  - `You are overweight. You should see your doctor.` when `classification` is `overweight`

Examples:

- `calculate_bmi_tenths(144, 72)` returns `195`
- `classify_bmi(195)` returns `ideal`
- `format_bmi_report(195, "ideal")` returns:
  - `Your BMI is 19.5.`
  - `You are within the ideal weight range.`
- `calculate_bmi_tenths(178, 62)` returns `325`
- `classify_bmi(325)` returns `overweight`
- `format_bmi_report(325, "overweight")` returns:
  - `Your BMI is 32.5.`
  - `You are overweight. You should see your doctor.`
- `calculate_bmi_tenths(100, 68)` returns `152`
- `classify_bmi(152)` returns `underweight`

## Non-Goals

This project does not include:

- persistence
- localization
- authentication
- network access
- configuration files
- validation for zero or negative height or weight
- decimal height or weight input values
- metric-unit input
- age-specific or gender-specific BMI ranges
- medical advice beyond the exercise's fixed doctor-consult message

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should adapt the same BMI-calculation, classification, and report-formatting behavior instead of redefining it.

Adapters should parse the prompted height and weight into whole numbers before calling the core logic.

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns `calculate_bmi_tenths`, `classify_bmi`, and `format_bmi_report`, and adapter repos must not reimplement them.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- tests for `calculate_bmi_tenths` on the canonical ideal-range example
- tests for an overweight example
- tests for an underweight example
- tests for the exact lower boundary `18.5`, which must classify as ideal
- tests for the exact upper boundary `25.0`, which must classify as ideal
- tests that `format_bmi_report` preserves the exact ideal two-line report
- tests that `format_bmi_report` preserves the exact underweight two-line report
- tests that `format_bmi_report` preserves the exact overweight two-line report
- tests for every adapter built in the chosen run that prove it delegates to the core logic correctly
