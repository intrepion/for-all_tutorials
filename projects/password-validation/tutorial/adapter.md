<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../README.md) / [Projects](../../../README.md) / [Password Validation](../../README.md) / [Tutorial](../README.md) / adapter.md
<!-- breadcrumbs:end -->

# Adapter Walkthrough

Project-specific red, green, refactor sequence for the `password-validation` adapter repo.

### 1. Red: Add An Adapter-Level Test

In a separate adapter repo, add a failing test that proves the chosen adapter collects the password input and delegates password validation and message formatting to the core logic correctly.

### 2. Green: Add The Thin Surface Adapter

Add the thinnest possible adapter for the surface you are building:

- collect the password input for the chosen surface
- if the chosen surface also collects a username, keep that username handling outside the core logic
- pass the raw password exactly as entered to `is_known_password`
- pass the resulting boolean value to `format_password_validation_message`
- render, return, or print the formatted result in the form that surface requires
- keep transport and input/output code out of the core password-validation logic

For prompt-driven adapters, the password prompt should be equivalent to:

```text
What is the password?
```

### 3. Refactor

Clean up any remaining duplication while keeping the full suite green.
