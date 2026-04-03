---
partial_kind: project-spec
project: numbers-to-names
---

# Spec

Canonical project contract for the `numbers-to-names` project.

## Goal

Build a small project app that introduces:

- converting a prompted month number into a numeric value
- mapping values from `1` through `12` to the corresponding calendar month names
- branching to an error message for out-of-range values
- separating month lookup from output formatting
- test-first mapping and formatting logic in small core functions
- thin surface adapters

## Core Logic Contract

The shared core contracts are:

```text
lookup_month_name(month_number: integer) -> month_name | null
format_month_lookup_message(month_name: string | null) -> string
```

Canonical behavior:

- treat `month_number` as a whole-number month candidate
- `lookup_month_name` returns:
  - `January` for `1`
  - `February` for `2`
  - `March` for `3`
  - `April` for `4`
  - `May` for `5`
  - `June` for `6`
  - `July` for `7`
  - `August` for `8`
  - `September` for `9`
  - `October` for `10`
  - `November` for `11`
  - `December` for `12`
  - `null` for any value outside `1` through `12`
- `format_month_lookup_message` returns:
  - `The name of the month is <month_name>.` when `month_name` is not `null`
  - `That is not a valid month number.` when `month_name` is `null`

Examples:

- `lookup_month_name(3)` returns `March`
- `format_month_lookup_message("March")` returns `The name of the month is March.`
- `lookup_month_name(1)` returns `January`
- `lookup_month_name(12)` returns `December`
- `lookup_month_name(13)` returns `null`
- `format_month_lookup_message(null)` returns `That is not a valid month number.`

## Non-Goals

This project does not include:

- persistence
- localization
- authentication
- network access
- configuration files
- validation for non-numeric input before parsing
- support for abbreviated month names
- support for zero-based month indexing
- localization or alternate languages for month names

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should adapt the same month-lookup and message-formatting behavior instead of redefining it.

Adapters should parse the prompted month number into a whole number before calling the core logic.

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns `lookup_month_name` and `format_month_lookup_message`, and adapter repos must not reimplement them.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- tests for a middle-of-range case such as `3 -> March`
- tests for the lower boundary `1 -> January`
- tests for the upper boundary `12 -> December`
- tests for an out-of-range case such as `13 -> null`
- tests that `format_month_lookup_message` preserves the exact success sentence
- tests that `format_month_lookup_message` preserves the exact error sentence
- tests for every adapter built in the chosen run that prove it delegates to the core logic correctly

