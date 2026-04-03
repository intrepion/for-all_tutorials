<!-- breadcrumbs:start -->
[for-all_tutorials](../../../README.md) / [Projects](../../README.md) / [Legal Driving Age](../README.md) / Spec
<!-- breadcrumbs:end -->

# Spec

Canonical project contract for the `legal-driving-age` project.

## Goal

Build a small project app that introduces:

- converting a prompted age into a numeric value
- comparing that age to the fixed legal driving threshold of `16`
- branching to one of two exact eligibility messages
- separating age comparison from output formatting
- test-first comparison and formatting logic in small core functions
- thin surface adapters

## Core Logic Contract

The shared core contracts are:

```text
is_old_enough_to_drive(age: integer) -> boolean
format_driving_eligibility_message(is_old_enough: boolean) -> string
```

Canonical behavior:

- treat `age` as a whole-number age
- `is_old_enough_to_drive` returns:
  - `true` when `age` is `16` or greater
  - `false` when `age` is less than `16`
- `format_driving_eligibility_message` returns:
  - `You are old enough to legally drive.` when `is_old_enough` is `true`
  - `You are not old enough to legally drive.` when `is_old_enough` is `false`
- the legal threshold is fixed at `16`

Examples:

- `is_old_enough_to_drive(15)` returns `false`
- `format_driving_eligibility_message(false)` returns `You are not old enough to legally drive.`
- `is_old_enough_to_drive(16)` returns `true`
- `format_driving_eligibility_message(true)` returns `You are old enough to legally drive.`
- `is_old_enough_to_drive(35)` returns `true`

## Non-Goals

This project does not include:

- persistence
- localization
- authentication
- network access
- configuration files
- validation for negative ages
- region-specific legal ages
- learner permits, restricted licenses, or other driving-rule variants
- custom threshold configuration

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should adapt the same age-comparison and message-formatting behavior instead of redefining it.

Adapters should parse the prompted age into a whole number before calling the core logic.

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns `is_old_enough_to_drive` and `format_driving_eligibility_message`, and adapter repos must not reimplement them.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- tests for an under-threshold case such as `15`
- tests for the exact threshold case `16`
- tests for an above-threshold case such as `35`
- tests that `format_driving_eligibility_message` returns the exact underage sentence
- tests that `format_driving_eligibility_message` returns the exact eligible sentence
- tests for every adapter built in the chosen run that prove it delegates to the core logic correctly
