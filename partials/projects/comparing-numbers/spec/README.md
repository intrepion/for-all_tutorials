---
partial_kind: project-spec
project: comparing-numbers
---

# Spec

Canonical project contract for the `comparing-numbers` project.

## Goal

Build a small project app that introduces:

- collecting a fixed set of three numeric inputs
- checking that all three numbers are different before continuing
- selecting the largest number only when the inputs are all distinct
- separating distinctness checks, largest-number comparison, and output formatting
- test-first comparison and formatting logic in small core functions
- thin surface adapters

## Core Logic Contract

The shared core contracts are:

```text
all_numbers_are_different(first_number: integer, second_number: integer, third_number: integer) -> boolean
largest_of_three(first_number: integer, second_number: integer, third_number: integer) -> integer
format_largest_number_message(largest_number: integer) -> string
```

Canonical behavior:

- `all_numbers_are_different` returns:
  - `true` when no two inputs are equal
  - `false` when any two inputs are equal
- `largest_of_three` returns the numerically largest input value
- `largest_of_three` assumes the caller has already confirmed that all three numbers are different
- `format_largest_number_message` returns:
  - `The largest number is <largest_number>.`

Examples:

- `all_numbers_are_different(1, 51, 2)` returns `true`
- `all_numbers_are_different(1, 1, 2)` returns `false`
- `all_numbers_are_different(2, 3, 2)` returns `false`
- `largest_of_three(1, 51, 2)` returns `51`
- `largest_of_three(51, 1, 2)` returns `51`
- `largest_of_three(1, 2, 51)` returns `51`
- `format_largest_number_message(51)` returns `The largest number is 51.`

## Non-Goals

This project does not include:

- persistence
- localization
- authentication
- network access
- configuration files
- input parsing or validation in the core
- sorting arbitrary-length collections
- handling ties beyond stopping the run before largest-number selection
- reporting multiple largest values

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should adapt the same distinctness check, largest-number selection, and success-message behavior instead of redefining it.

Adapters should:

- prompt for exactly three numbers
- parse those prompted values into whole numbers before calling the core logic
- call `all_numbers_are_different` before calling `largest_of_three`
- stop the current run when `all_numbers_are_different` returns `false`
- not call `largest_of_three` or render `format_largest_number_message` when the inputs are not all different
- call `largest_of_three` and then `format_largest_number_message` when the inputs are all different

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns `all_numbers_are_different`, `largest_of_three`, and `format_largest_number_message`, and adapter repos must not reimplement them.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- tests that `all_numbers_are_different` returns `true` for the canonical `1, 51, 2` example
- tests that `all_numbers_are_different` returns `false` when the first two numbers match
- tests that `all_numbers_are_different` returns `false` when the first and third numbers match
- tests that `largest_of_three` returns the correct value when the largest number is first, second, or third
- tests that `format_largest_number_message` preserves the exact canonical sentence
- tests for every adapter built in the chosen run that prove it checks distinctness before selecting the largest number and that it stops without rendering the success message when duplicate inputs are provided

