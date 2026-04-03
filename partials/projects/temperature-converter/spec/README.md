---
partial_kind: project-spec
project: temperature-converter
---

# Spec

Canonical project contract for the `temperature-converter` project.

## Goal

Build a small project app that introduces:

- converting a prompted temperature into a numeric value
- branching on a conversion choice
- applying one of two temperature-conversion formulas
- rounding the converted temperature to the nearest tenth with half-up rounding
- separating conversion calculation from output formatting
- test-first conversion and formatting logic in small core functions
- thin surface adapters

## Core Logic Contract

The shared core contracts are:

```text
convert_temperature_tenths(starting_temperature_tenths: integer, conversion_choice: string) -> converted_temperature_tenths
format_temperature_conversion_report(conversion_choice: string, converted_temperature_tenths: integer) -> string
```

Canonical behavior:

- treat `starting_temperature_tenths` as the input temperature scaled by `10`, where:
  - `320` means `32.0`
  - `1005` means `100.5`
- treat `conversion_choice` as one of two exact values:
  - `C`, which means convert from Fahrenheit to Celsius
  - `F`, which means convert from Celsius to Fahrenheit
- `convert_temperature_tenths` returns:
  - for `C`: `C = (F - 32) * (5 / 9)`
  - for `F`: `F = (C * (9 / 5)) + 32`
  - the converted temperature rounded to the nearest tenth with half-up rounding
- `format_temperature_conversion_report` returns:
  - `The temperature in Celsius is <formatted_temperature>.` when `conversion_choice` is `C`
  - `The temperature in Fahrenheit is <formatted_temperature>.` when `conversion_choice` is `F`
- `<formatted_temperature>`:
  - omits the decimal portion when the tenths digit is `0`
  - otherwise renders exactly one decimal place

Examples:

- `convert_temperature_tenths(320, "C")` returns `0`
- `format_temperature_conversion_report("C", 0)` returns `The temperature in Celsius is 0.`
- `convert_temperature_tenths(1000, "C")` returns `378`
- `format_temperature_conversion_report("C", 378)` returns `The temperature in Celsius is 37.8.`
- `convert_temperature_tenths(0, "F")` returns `320`
- `format_temperature_conversion_report("F", 320)` returns `The temperature in Fahrenheit is 32.`

## Non-Goals

This project does not include:

- persistence
- localization
- authentication
- network access
- configuration files
- validation for unsupported conversion choices
- choice normalization, trimming, or case folding
- Kelvin conversion
- preserving more than one decimal place in the converted output

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should adapt the same conversion and report-formatting behavior instead of redefining it.

Adapters should parse the prompted temperature into tenths and pass the exact conversion choice expected by the core logic.

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns `convert_temperature_tenths` and `format_temperature_conversion_report`, and adapter repos must not reimplement them.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- tests for the canonical Fahrenheit-to-Celsius example where `32` becomes `0`
- tests for a non-whole Fahrenheit-to-Celsius case such as `100` becoming `37.8`
- tests for a Celsius-to-Fahrenheit case such as `0` becoming `32`
- tests that `format_temperature_conversion_report` preserves the exact Celsius sentence
- tests that `format_temperature_conversion_report` preserves the exact Fahrenheit sentence
- tests that the formatter omits the decimal portion when the converted value is a whole number
- tests for every adapter built in the chosen run that prove it delegates to the core logic correctly

