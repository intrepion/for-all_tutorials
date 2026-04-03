<!-- breadcrumbs:start -->
[for-all_tutorials](../../../README.md) / [Projects](../../README.md) / [Adding Numbers](../README.md) / Spec
<!-- breadcrumbs:end -->

# Spec

Canonical project contract for the `adding-numbers` project.

## Goal

Build a small project app that introduces:

- converting repeated plain-text inputs into numeric values
- summing a fixed list of five whole numbers
- separating numeric accumulation from output formatting
- keeping the "prompt five times" rule in the adapter while the core stays reusable
- test-first calculation and formatting logic in small core functions
- thin surface adapters

## Core Logic Contract

The shared core contracts are:

```text
sum_numbers(numbers: integer[]) -> integer
format_total_report(total: integer) -> string
```

Canonical behavior:

- treat `numbers` as a list of signed whole numbers
- for this project, the adapter must collect exactly five numbers before calling the core
- `sum_numbers` returns the arithmetic total of every value in `numbers`
- `format_total_report` returns:
  - `The total is <total>.`
- the report must preserve the computed total exactly as supplied

Examples:

- `sum_numbers([1, 2, 3, 4, 5])` returns `15`
- `format_total_report(15)` returns `The total is 15.`
- `sum_numbers([10, -3, 0, 8, 5])` returns `20`

## Non-Goals

This project does not include:

- persistence
- localization
- authentication
- network access
- configuration files
- variable-length input lists
- floating-point parsing or rounding rules
- statistics beyond the total, such as average, min, or max
- expression parsing beyond one prompted number at a time

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should adapt the same summing and report-formatting behavior instead of redefining it.

Adapters should prompt for exactly five numbers, parse them into whole numbers, preserve their order, and pass the resulting list to `sum_numbers`.

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns `sum_numbers` and `format_total_report`, and adapter repos must not reimplement them.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- tests for `sum_numbers` on the canonical `[1, 2, 3, 4, 5] -> 15` example
- tests for mixed signed values such as `[10, -3, 0, 8, 5] -> 20`
- tests that `format_total_report` preserves the exact canonical sentence
- tests for every adapter built in the chosen run that prove it prompts five times and delegates to the core logic correctly
