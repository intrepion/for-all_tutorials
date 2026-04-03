---
partial_kind: project-spec
project: multiplication-table
---

# Spec

Canonical project contract for the `multiplication-table` project.

## Goal

Build a small project app that introduces:

- generating a complete multiplication table for a fixed numeric range
- using nested iteration to produce every multiplication pair
- separating table generation from output formatting
- preserving a stable row-major output order
- test-first generation and formatting logic in small core functions
- thin surface adapters

## Core Logic Contract

The shared core contracts are:

```text
generate_multiplication_table() -> rows
format_multiplication_table(rows) -> string[]
```

Where each row in `rows` contains:

```text
left_factor
right_factor
product
```

Canonical behavior:

- `generate_multiplication_table` returns every multiplication combination from `0` through `12`, inclusive
- use row-major order:
  - `left_factor` runs from `0` through `12`
  - for each `left_factor`, `right_factor` runs from `0` through `12`
- each row contains:
  - `left_factor`
  - `right_factor`
  - `product = left_factor * right_factor`
- `format_multiplication_table` returns one line per row in this exact format:
  - `<left_factor> x <right_factor> = <product>`
- the formatted output must preserve the exact row order returned by `generate_multiplication_table`

Examples:

- the first generated row is `{ left_factor: 0, right_factor: 0, product: 0 }`
- the second generated row is `{ left_factor: 0, right_factor: 1, product: 0 }`
- the last generated row is `{ left_factor: 12, right_factor: 12, product: 144 }`
- `format_multiplication_table([{ left_factor: 0, right_factor: 0, product: 0 }])` returns:
  - `0 x 0 = 0`
- the last formatted line in the complete table is:
  - `12 x 12 = 144`

## Non-Goals

This project does not include:

- persistence
- localization
- authentication
- network access
- configuration files
- user-supplied table ranges
- alternate multiplication symbols or layout styles
- grid or column alignment formatting
- filtering or partial table generation in the adapter

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should adapt the same table-generation and table-formatting behavior instead of redefining it.

Adapters should call `generate_multiplication_table`, pass the result to `format_multiplication_table`, and render the returned lines in order.

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns `generate_multiplication_table` and `format_multiplication_table`, and adapter repos must not reimplement them.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- tests that the generated table starts with `0 x 0`
- tests that the generated table ends with `12 x 12`
- tests that the generated table includes the correct product for a middle row such as `7 x 8 = 56`
- tests that the generated table contains exactly `169` rows
- tests that `format_multiplication_table` preserves the exact canonical line format
- tests for every adapter built in the chosen run that prove it delegates to the core logic correctly

