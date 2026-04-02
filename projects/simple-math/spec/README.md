<!-- breadcrumbs:start -->
[for-all_tutorials](../../../README.md) / [Projects](../../README.md) / [Simple Math](../README.md) / Spec
<!-- breadcrumbs:end -->

# Spec

Canonical project contract for the `simple-math` project.

## Goal

Build a small project app that introduces:

- converting two plain-text inputs into numeric values
- computing sum, difference, product, and quotient
- separating arithmetic from output formatting
- test-first calculation and formatting logic in small core functions
- thin surface adapters

## Core Logic Contract

The shared core contracts are:

```text
calculate_simple_math(first: integer, second: integer) -> { sum, difference, product, quotient }
format_simple_math_report(first: integer, second: integer, results) -> string[]
```

Canonical behavior:

- treat `first` and `second` as signed whole numbers
- `calculate_simple_math` returns:
  - `sum = first + second`
  - `difference = first - second`
  - `product = first * second`
  - `quotient = first / second`
- for this project, `quotient` is only defined when `second` is nonzero and `first` is evenly divisible by `second`
- `format_simple_math_report` returns four lines in this exact order:
  - `<first> + <second> = <sum>`
  - `<first> - <second> = <difference>`
  - `<first> * <second> = <product>`
  - `<first> / <second> = <quotient>`
- preserve the original numeric values in each rendered line

Examples:

- `calculate_simple_math(10, 5)` returns `{ sum: 15, difference: 5, product: 50, quotient: 2 }`
- `format_simple_math_report(10, 5, { sum: 15, difference: 5, product: 50, quotient: 2 })` returns:
  - `10 + 5 = 15`
  - `10 - 5 = 5`
  - `10 * 5 = 50`
  - `10 / 5 = 2`
- `calculate_simple_math(-6, 3)` returns `{ sum: -3, difference: -9, product: -18, quotient: -2 }`

## Non-Goals

This project does not include:

- persistence
- localization
- authentication
- network access
- configuration files
- divide-by-zero handling
- non-integer quotients
- floating-point parsing or rounding rules
- expression parsing beyond two prompted inputs

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should adapt the same arithmetic and report-formatting behavior instead of redefining it.

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns `calculate_simple_math` and `format_simple_math_report`, and adapter repos must not reimplement them.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- tests for `calculate_simple_math` on the canonical example
- tests for negative-number behavior
- tests that `format_simple_math_report` preserves the exact line order and operators
- tests that the four report lines include the original operands and computed results
- tests for every adapter built in the chosen run that prove it delegates to the core logic correctly
