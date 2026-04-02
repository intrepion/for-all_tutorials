<!-- breadcrumbs:start -->
[for-all_tutorials](../../../README.md) / [Projects](../../README.md) / [Pizza Party](../README.md) / Spec
<!-- breadcrumbs:end -->

# Spec

Canonical project contract for the `pizza-party` project.

## Goal

Build a small project app that introduces:

- converting three prompted counts into numeric values
- calculating total pizza slices
- using whole-number division to compute slices per person
- using remainder to compute leftover slices
- separating distribution calculation from output formatting
- test-first calculation and formatting logic in small core functions
- thin surface adapters

## Core Logic Contract

The shared core contracts are:

```text
calculate_pizza_distribution(people: integer, pizzas: integer, slices_per_pizza: integer) -> { total_slices, slices_per_person, leftover_slices }
format_pizza_distribution_report(people: integer, pizzas: integer, distribution) -> string[]
```

Canonical behavior:

- treat `people`, `pizzas`, and `slices_per_pizza` as positive whole numbers
- `calculate_pizza_distribution` returns:
  - `total_slices = pizzas * slices_per_pizza`
  - `slices_per_person = total_slices / people` using whole-number division
  - `leftover_slices = total_slices % people`
- `format_pizza_distribution_report` returns three lines in this exact order:
  - `<people> people with <pizzas> pizzas`
  - `Each person gets <slices_per_person> pieces of pizza.`
  - `There are <leftover_slices> leftover pieces.`
- preserve the original `people` and `pizzas` counts exactly in the first line

Examples:

- `calculate_pizza_distribution(8, 2, 8)` returns `{ total_slices: 16, slices_per_person: 2, leftover_slices: 0 }`
- `format_pizza_distribution_report(8, 2, { total_slices: 16, slices_per_person: 2, leftover_slices: 0 })` returns:
  - `8 people with 2 pizzas`
  - `Each person gets 2 pieces of pizza.`
  - `There are 0 leftover pieces.`
- `calculate_pizza_distribution(8, 2, 9)` returns `{ total_slices: 18, slices_per_person: 2, leftover_slices: 2 }`

## Non-Goals

This project does not include:

- persistence
- localization
- authentication
- network access
- configuration files
- validation for zero or negative counts
- grammatical singular-versus-plural wording changes
- uneven pizza sizes or multiple slice counts per pizza
- optimization beyond even whole-slice distribution

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should adapt the same pizza-distribution and report-formatting behavior instead of redefining it.

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns `calculate_pizza_distribution` and `format_pizza_distribution_report`, and adapter repos must not reimplement them.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- tests for `calculate_pizza_distribution` on the canonical example
- tests for a leftover-slices case where the division is not even
- tests that `format_pizza_distribution_report` preserves the exact line order and wording
- tests that the original `people` and `pizzas` counts appear unchanged in the first line
- tests for every adapter built in the chosen run that prove it delegates to the core logic correctly
