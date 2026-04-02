<!-- breadcrumbs:start -->
[for-all_tutorials](../../../README.md) / [Projects](../../README.md) / [Area of a Rectangular Room](../README.md) / Spec
<!-- breadcrumbs:end -->

# Spec

Canonical project contract for the `area-of-a-rectangular-room` project.

## Goal

Build a small project app that introduces:

- converting two prompted room dimensions into numeric values
- calculating area in square feet
- converting square feet to square meters with a fixed conversion constant
- separating area calculation from output formatting
- test-first calculation and formatting logic in small core functions
- thin surface adapters

## Core Logic Contract

The shared core contracts are:

```text
calculate_room_area(length_feet: integer, width_feet: integer) -> { square_feet, square_meters }
format_room_area_report(length_feet: integer, width_feet: integer, area) -> string[]
```

Canonical behavior:

- treat `length_feet` and `width_feet` as whole-number room dimensions in feet
- `calculate_room_area` returns:
  - `square_feet = length_feet * width_feet`
  - `square_meters = square_feet * 0.09290304`
- `format_room_area_report` returns four lines in this exact order:
  - `You entered dimensions of <length_feet> feet by <width_feet> feet.`
  - `The area is`
  - `<square_feet> square feet`
  - `<square_meters_rounded_to_3_decimal_places> square meters`
- preserve the original feet dimensions exactly in the first line
- round the displayed square-meter value to exactly three decimal places

Examples:

- `calculate_room_area(15, 20)` returns `{ square_feet: 300, square_meters: 27.870912 }`
- `format_room_area_report(15, 20, { square_feet: 300, square_meters: 27.870912 })` returns:
  - `You entered dimensions of 15 feet by 20 feet.`
  - `The area is`
  - `300 square feet`
  - `27.871 square meters`
- `calculate_room_area(10, 12)` returns `{ square_feet: 120, square_meters: 11.1483648 }`

## Non-Goals

This project does not include:

- persistence
- localization
- authentication
- network access
- configuration files
- validation for zero or negative dimensions
- decimal room dimensions in feet
- metric input
- alternate conversion formulas or unit systems

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should adapt the same area-calculation and report-formatting behavior instead of redefining it.

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns `calculate_room_area` and `format_room_area_report`, and adapter repos must not reimplement them.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- tests for `calculate_room_area` on the canonical example
- tests that the square-meter conversion uses the exact constant `0.09290304`
- tests that `format_room_area_report` preserves the exact line order and wording
- tests that the displayed square-meter value is rounded to three decimal places
- tests for every adapter built in the chosen run that prove it delegates to the core logic correctly
