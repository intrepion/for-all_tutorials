<!-- breadcrumbs:start -->
[for-all_tutorials](../../../README.md) / [Projects](../../README.md) / [Password Validation](../README.md) / Spec
<!-- breadcrumbs:end -->

# Spec

Canonical project contract for the `password-validation` project.

## Goal

Build a small project app that introduces:

- comparing one entered password to one fixed known password
- branching to one of two exact result messages
- separating password comparison from output formatting
- test-first validation and formatting logic in small core functions
- thin surface adapters

## Core Logic Contract

The shared core contracts are:

```text
is_known_password(password: string) -> boolean
format_password_validation_message(is_valid: boolean) -> string
```

Canonical behavior:

- treat `password` as the exact plain-text value supplied to the core logic
- do not trim, normalize, or case-fold `password`
- the fixed known password is `abc$123`
- `is_known_password` returns:
  - `true` when `password` is exactly `abc$123`
  - `false` for every other value
- `format_password_validation_message` returns:
  - `Welcome!` when `is_valid` is `true`
  - `I don't know you.` when `is_valid` is `false`

Examples:

- `is_known_password("12345")` returns `false`
- `format_password_validation_message(false)` returns `I don't know you.`
- `is_known_password("abc$123")` returns `true`
- `format_password_validation_message(true)` returns `Welcome!`

## Non-Goals

This project does not include:

- persistence
- localization
- authentication sessions
- network access
- configuration files
- password hashing or secret storage
- username validation in the core
- account locking or rate limiting
- case-insensitive password comparison
- password rules beyond the exact known-password match

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should adapt the same password-comparison and message-formatting behavior instead of redefining it.

Adapters should:

- prompt for a password
- pass the raw password exactly as entered to `is_known_password`
- pass the boolean result to `format_password_validation_message`
- render, return, or print the formatted result in the form that surface requires

Adapters may also collect a username, but the username does not affect the core rule for this project.

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns `is_known_password` and `format_password_validation_message`, and adapter repos must not reimplement them.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- tests that `is_known_password` returns `false` for the canonical invalid example `12345`
- tests that `is_known_password` returns `true` for the canonical valid example `abc$123`
- tests that `format_password_validation_message` preserves the exact welcome sentence
- tests that `format_password_validation_message` preserves the exact rejection sentence
- tests for every adapter built in the chosen run that prove it delegates to the core logic correctly
