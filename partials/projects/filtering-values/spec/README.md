---
partial_kind: project-spec
project: filtering-values
---

# Spec

Canonical project contract for the `filtering-values` project.

## Goal

Build a small project app that introduces:

- collecting a space-separated list of numbers
- preserving input order while filtering a list
- keeping only the even values from the entered list
- separating even-number filtering and output formatting from input parsing
- test-first list-filtering and formatting logic in small core functions
- thin surface adapters

## Core Logic Contract

The shared core contracts are:

```text
filter_even_numbers(numbers: integer[]) -> integer[]
format_even_numbers_report(even_numbers: integer[]) -> string
```

Canonical behavior:

- treat `numbers` as a list of signed whole numbers
- `filter_even_numbers`:
  - returns a new list
  - preserves the original relative order of all kept values
  - keeps a value when it is evenly divisible by `2`
  - removes a value when it is not evenly divisible by `2`
- `format_even_numbers_report` returns:
  - `The even numbers are <values joined by single spaces>.` when `even_numbers` is not empty
  - `The even numbers are.` when `even_numbers` is empty

Examples:

- `filter_even_numbers([1, 2, 3, 4, 5, 6, 7, 8])` returns `[2, 4, 6, 8]`
- `format_even_numbers_report([2, 4, 6, 8])` returns `The even numbers are 2 4 6 8.`
- `filter_even_numbers([1, 3, 5])` returns `[]`
- `format_even_numbers_report([])` returns `The even numbers are.`
- `filter_even_numbers([-4, -3, 0, 7, 12])` returns `[-4, 0, 12]`

## Non-Goals

This project does not include:

- persistence
- localization
- authentication
- network access
- configuration files
- raw string parsing in the core
- decimal numbers
- sorting or reordering values
- deduplicating values
- filtering rules beyond even-number selection

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should adapt the same even-number filtering and report-formatting behavior instead of redefining it.

Adapters should:

- prompt for one line of numbers separated by spaces
- split the entered line on spaces
- parse the resulting values into whole numbers before calling the core
- preserve the original numeric order
- pass the parsed list to `filter_even_numbers`
- pass the returned list to `format_even_numbers_report`
- render, return, or print the formatted report in the form that surface requires

For prompt-driven adapters, the prompt should be equivalent to:

```text
Enter a list of numbers, separated by spaces:
```

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns `filter_even_numbers` and `format_even_numbers_report`, and adapter repos must not reimplement them.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- a test that `filter_even_numbers([1, 2, 3, 4, 5, 6, 7, 8])` returns `[2, 4, 6, 8]`
- a test that `filter_even_numbers` preserves order for mixed signed values such as `[-4, -3, 0, 7, 12] -> [-4, 0, 12]`
- a test that `filter_even_numbers` returns an empty list when no values are even
- a test that `format_even_numbers_report` preserves the exact canonical sentence for a non-empty result
- a test that `format_even_numbers_report` preserves the defined empty-result sentence
- tests for every adapter built in the chosen run that prove it parses the space-separated input, preserves order, and delegates filtering and formatting to the core logic correctly

