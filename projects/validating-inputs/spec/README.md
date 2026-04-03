<!-- breadcrumbs:start -->
[for-all_tutorials](../../../README.md) / [Projects](../../README.md) / [Validating Inputs](../README.md) / Spec
<!-- breadcrumbs:end -->

# Spec

Canonical project contract for the `validating-inputs` project.

## Goal

Build a small project app that introduces:

- validating multiple prompted fields with single-purpose functions
- separating field-level validation from error aggregation and report formatting
- checking required values, minimum lengths, numeric-only input, and fixed-format identifiers
- collecting all validation errors in a stable order
- test-first validation and formatting logic in small core functions
- thin surface adapters

## Core Logic Contract

The shared core contracts are:

```text
validate_first_name(first_name: string) -> error | null
validate_last_name(last_name: string) -> error | null
validate_zip_code(zip_code: string) -> error | null
validate_employee_id(employee_id: string) -> error | null
collect_validation_errors(first_name: string, last_name: string, zip_code: string, employee_id: string) -> string[]
format_validation_report(errors: string[]) -> string[]
```

Canonical behavior:

- treat all inputs as plain-text ASCII strings
- do not trim, normalize, or rewrite any input before validation
- `validate_first_name` returns:
  - `The first name must be filled in.` when `first_name` is empty
  - `"<first_name>" is not a valid first name. It is too short.` when `first_name` has length `1`
  - `null` otherwise
- `validate_last_name` returns:
  - `The last name must be filled in.` when `last_name` is empty
  - `"<last_name>" is not a valid last name. It is too short.` when `last_name` has length `1`
  - `null` otherwise
- `validate_zip_code` returns:
  - `The ZIP code must be numeric.` when `zip_code` contains any non-digit character or is empty
  - `null` otherwise
- `validate_employee_id` returns:
  - `null` when `employee_id` matches this exact ASCII pattern:
    - two letters
    - one hyphen
    - four digits
  - `employee_id` may use uppercase or lowercase ASCII letters
  - `employee_id` is invalid for any other shape
  - `A12-1234 is not a valid ID.` when the value is invalid
- `collect_validation_errors`:
  - calls the four field validators
  - returns only the non-null error messages
  - preserves this exact error order:
    - first name
    - last name
    - ZIP code
    - employee ID
- `format_validation_report` returns:
  - the collected error lines unchanged when `errors` is non-empty
  - one line, `There were no errors found.`, when `errors` is empty

Examples:

- `validate_first_name("J")` returns `"J" is not a valid first name. It is too short.`
- `validate_last_name("")` returns `The last name must be filled in.`
- `validate_zip_code("ABCDE")` returns `The ZIP code must be numeric.`
- `validate_employee_id("A12-1234")` returns `A12-1234 is not a valid ID.`
- `collect_validation_errors("J", "", "ABCDE", "A12-1234")` returns:
  - `"J" is not a valid first name. It is too short.`
  - `The last name must be filled in.`
  - `The ZIP code must be numeric.`
  - `A12-1234 is not a valid ID.`
- `collect_validation_errors("Jimmy", "James", "55555", "TK-4210")` returns an empty list
- `format_validation_report([])` returns:
  - `There were no errors found.`

## Non-Goals

This project does not include:

- persistence
- localization
- authentication
- network access
- configuration files
- input trimming or whitespace normalization
- ZIP+4 support
- international postal-code rules
- non-ASCII name validation
- custom employee ID formats beyond the one fixed pattern
- stopping after the first validation error

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should adapt the same field-validation, error-aggregation, and report-formatting behavior instead of redefining it.

Adapters should collect the four prompted strings and pass them through the core validation flow without rewriting them first.

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns the field validators, `collect_validation_errors`, and `format_validation_report`, and adapter repos must not reimplement them.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- tests for empty and one-character first-name cases
- tests for empty and one-character last-name cases
- tests that `validate_zip_code` rejects non-numeric input
- tests that `validate_employee_id` rejects malformed IDs and accepts valid IDs
- tests that `collect_validation_errors` returns the canonical four-line invalid example in the correct order
- tests that `format_validation_report` preserves the error list unchanged
- tests that `format_validation_report` returns `There were no errors found.` for an empty error list
- tests for every adapter built in the chosen run that prove it delegates to the core logic correctly
