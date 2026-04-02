<!-- breadcrumbs:start -->
[for-all_tutorials](../../../README.md) / [Projects](../../README.md) / [Paint Calculator](../README.md) / Spec
<!-- breadcrumbs:end -->

# Spec

Canonical project contract for the `paint-calculator` project.

## Goal

Build a small project app that introduces:

- converting two prompted room dimensions into numeric values
- calculating ceiling area in square feet
- dividing ceiling area by a fixed coverage rate of `350` square feet per gallon
- rounding up to the next whole gallon whenever the area is not an exact multiple of `350`
- separating paint-requirement calculation from output formatting
- test-first calculation and formatting logic in small core functions
- thin surface adapters

## Core Logic Contract

The shared core contracts are:

```text
calculate_paint_requirements(length_feet: integer, width_feet: integer) -> { square_feet, gallons_needed }
format_paint_purchase_message(square_feet: integer, gallons_needed: integer) -> string
```

Canonical behavior:

- treat `length_feet` and `width_feet` as whole-number room dimensions in feet
- `calculate_paint_requirements` returns:
  - `square_feet = length_feet * width_feet`
  - `gallons_needed = ceiling(square_feet / 350)`
- `format_paint_purchase_message` returns:
  - `You will need to purchase 1 gallon of paint to cover <square_feet> square feet.` when `gallons_needed` is `1`
  - `You will need to purchase <gallons_needed> gallons of paint to cover <square_feet> square feet.` when `gallons_needed` is greater than `1`
- preserve the computed `square_feet` and `gallons_needed` exactly in the rendered output
- never round down and never use standard nearest-number rounding for `gallons_needed`

Examples:

- `calculate_paint_requirements(15, 24)` returns `{ square_feet: 360, gallons_needed: 2 }`
- `format_paint_purchase_message(360, 2)` returns `You will need to purchase 2 gallons of paint to cover 360 square feet.`
- `calculate_paint_requirements(14, 25)` returns `{ square_feet: 350, gallons_needed: 1 }`
- `format_paint_purchase_message(350, 1)` returns `You will need to purchase 1 gallon of paint to cover 350 square feet.`
- `calculate_paint_requirements(13, 27)` returns `{ square_feet: 351, gallons_needed: 2 }`

## Non-Goals

This project does not include:

- persistence
- localization
- authentication
- network access
- configuration files
- validation for zero or negative dimensions
- decimal room dimensions in feet
- painting walls, doors, trim, or multiple rooms
- multiple coats, primer calculations, or waste estimates
- alternate coverage rates beyond `350` square feet per gallon

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should adapt the same paint-requirement and purchase-message behavior instead of redefining it.

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns `calculate_paint_requirements` and `format_paint_purchase_message`, and adapter repos must not reimplement them.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- tests for `calculate_paint_requirements` on the canonical example
- tests for an exact-coverage case where `350` square feet still requires only `1` gallon
- tests for a just-over-the-boundary case where `351` square feet requires `2` gallons
- tests that `format_paint_purchase_message` preserves the exact wording for both the singular and plural gallon cases
- tests for every adapter built in the chosen run that prove it delegates to the core logic correctly
